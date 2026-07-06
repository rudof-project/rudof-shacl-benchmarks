import gc
import time
from concurrent.futures import ThreadPoolExecutor, TimeoutError as FutureTimeoutError
from dataclasses import replace

from args import Args
from results import IterationResults, IterationResultStatus, ResultState
from validation_engine import ValidationEngine

class BenchmarkRunner:
    def __init__(self, engine: ValidationEngine, args: Args):
        self._engine = engine
        self._args = args

    def run(self) -> ResultState:
        results = ResultState(self._args.min_valid_iter)

        for idx in range(self._args.warm_up + self._args.runs):
            iteration_result, report = self._run_iteration(idx)

            if idx >= self._args.warm_up:
                results.add_result(replace(iteration_result, iteration=iteration_result.iteration - self._args.warm_up))
                if report is not None:
                    with open(self._args.report_path, "w", encoding="utf-8") as f:
                        f.write(report)

            if idx == self._args.warm_up - 1:
                print(f"[{self._engine.name}] Warm-up complete")

        return results

    def _run_iteration(self, idx: int) -> tuple[IterationResults, str | None]:
        executor = ThreadPoolExecutor(max_workers=1, thread_name_prefix=f"{self._engine.name}-run-{idx}")
        future = executor.submit(self._iteration_body)

        try:
            load_ms, validation_ms, report = future.result(timeout=self._args.timeout)
            return IterationResults(idx, load_ms, validation_ms, IterationResultStatus.OK), report
        except FutureTimeoutError:
            print(f"[{self._engine.name}] Run {idx} exceeded timeout of {self._args.timeout}s")
            return IterationResults(idx, None, None, IterationResultStatus.DNF), None
        except MemoryError as e:
            print(f"[{self._engine.name}] Run {idx} failed: {e}")
            return IterationResults(idx, None, None, IterationResultStatus.OOM), None
        except Exception as e:
            print(f"[{self._engine.name}] Run {idx} failed: {e}")
            return IterationResults(idx, None, None, IterationResultStatus.ERROR), None
        finally:
            executor.shutdown(wait=False, cancel_futures=True)

    def _iteration_body(self) -> tuple[float, float, str]:
        gc.collect()
        gc.disable()
        load_start = time.perf_counter_ns()
        self._engine.load_data(
            self._args.data_path,
            self._args.data_format,
            self._args.shapes_path,
            self._args.shapes_format,
        )
        load_ms = (time.perf_counter_ns() - load_start) / 1_000_000.0
        gc.enable()

        gc.collect()
        gc.disable()
        validation_start = time.perf_counter_ns()
        result = self._engine.validate()
        validation_ms = (time.perf_counter_ns() - validation_start) / 1_000_000.0
        gc.enable()

        report = self._engine.generate_report(result)
        return load_ms, validation_ms, report
