use criterion::profiler::Profiler;
use pprof::{ProfilerGuard, ProfilerGuardBuilder};
use std::ffi::c_int;
use std::fs::{create_dir_all, File};
use std::path::Path;

pub struct FlamegraphProfiler<'a> {
    freq: c_int,
    active_profiler: Option<ProfilerGuard<'a>>
}

impl<'a> FlamegraphProfiler<'a> {
    pub fn new(freq: c_int) -> Self {
        FlamegraphProfiler {
            freq,
            active_profiler: None
        }
    }
}

impl<'a> Profiler for FlamegraphProfiler<'a> {
    fn start_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {
        let guard = ProfilerGuardBuilder::default()
            .frequency(self.freq)
            .blocklist(&["libc", "libgcc", "pthread", "vdso"])
            .build()
            .unwrap();
        self.active_profiler = Some(guard);
    }

    fn stop_profiling(&mut self, _benchmark_id: &str, benchmark_dir: &Path) {
        create_dir_all(benchmark_dir).unwrap();

        let flamegraph_path = benchmark_dir.join("flamegraph.svg");
        let flamegraph_file = File::create(&flamegraph_path)
            .expect("File system error while creating flamegraph file");
        if let Some(profiler) = self.active_profiler.take() {
            profiler
                .report()
                .build()
                .unwrap()
                .flamegraph(flamegraph_file)
                .expect("Error writing flamegraph");
        }
    }
}