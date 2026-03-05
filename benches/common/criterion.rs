use crate::common::FlamegraphProfiler;
use criterion::Criterion;

pub fn criterion_config() -> Criterion {
    Criterion::default()
        .with_profiler(FlamegraphProfiler::new(100))
}