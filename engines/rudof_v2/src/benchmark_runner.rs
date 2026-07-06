use std::any::Any;
use std::fs;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};

use crate::args::Args;
use crate::results::{IterationResultStatus, IterationResults, ResultState};
use crate::validation_engine::ValidationEngine;

pub struct BenchmarkRunner<E, F>
where
    E: ValidationEngine + Send + 'static,
    E::Report: Send + 'static,
    F: Fn() -> E,
{
    factory: F,
    args: Args,
}

impl<E, F> BenchmarkRunner<E, F>
where
    E: ValidationEngine + Send + 'static,
    E::Report: Send + 'static,
    F: Fn() -> E,
{
    pub fn new(factory: F, args: Args) -> Self {
        Self { factory, args }
    }

    pub fn run(&self) -> ResultState {
        let mut state = ResultState::new(self.args.min_valid_iter);

        for idx in 0..(self.args.warm_up + self.args.runs) {
            let (result, report) = self.run_iteration(idx);

            if idx >= self.args.warm_up {
                state.add_result(IterationResults {
                    iteration: result.iteration - self.args.warm_up,
                    ..result
                });
                if let Some(r) = report {
                    fs::write(&self.args.report_path, r).expect("Failed to write validation report");
                }
            }
            if self.args.warm_up > 0 && idx == self.args.warm_up - 1 {
                println!("[{}] Warm-up complete", E::NAME);
            }
        }

        state
    }

    fn run_iteration(&self, idx: usize) -> (IterationResults, Option<String>) {
        let engine = (self.factory)();
        let data_path = self.args.data_path.clone();
        let data_format = self.args.data_format.clone();
        let shapes_path = self.args.shapes_path.clone();
        let shapes_format = self.args.shapes_format.clone();

        let (tx, rx) = mpsc::sync_channel(1);

        let _handle = thread::Builder::new()
            .name(format!("{}-run-{}", E::NAME, idx))
            .spawn(move || {
                let mut engine = engine;
                let outcome = catch_unwind(AssertUnwindSafe(|| {
                    let load_start = Instant::now();
                    engine.load_data(&data_path, &data_format, &shapes_path, &shapes_format);
                    let load_ms = load_start.elapsed().as_micros() as f64 / 1000.0;

                    let validation_start = Instant::now();
                    let result = engine.validate();
                    let validation_ms = validation_start.elapsed().as_micros() as f64 / 1000.0;

                    let report = engine.generate_report(result);
                    (load_ms, validation_ms, report)
                }));
                let _ = tx.send(outcome);
            })
            .expect("Failed to spawn iteration thread");

        match rx.recv_timeout(Duration::from_secs(self.args.timeout)) {
            Ok(Ok((load_ms, validation_ms, report))) => (
                IterationResults {
                    iteration: idx,
                    load_time: Some(load_ms),
                    validation_time: Some(validation_ms),
                    iteration_results: IterationResultStatus::Ok,
                },
                Some(report),
            ),
            Ok(Err(panic)) => {
                println!("[{}] Run {} failed: {}", E::NAME, idx, panic_message(&panic));
                (
                    IterationResults {
                        iteration: idx,
                        load_time: None,
                        validation_time: None,
                        iteration_results: IterationResultStatus::Error,
                    },
                    None,
                )
            }
            Err(RecvTimeoutError::Timeout) => {
                println!("[{}] Run {} exceeded timeout of {}s", E::NAME, idx, self.args.timeout);
                // Thread continues running; we detach and move on.
                (
                    IterationResults {
                        iteration: idx,
                        load_time: None,
                        validation_time: None,
                        iteration_results: IterationResultStatus::Dnf,
                    },
                    None,
                )
            }
            Err(RecvTimeoutError::Disconnected) => (
                IterationResults {
                    iteration: idx,
                    load_time: None,
                    validation_time: None,
                    iteration_results: IterationResultStatus::Error,
                },
                None,
            ),
        }
    }
}

fn panic_message(payload: &Box<dyn Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "unknown panic".to_string()
    }
}
