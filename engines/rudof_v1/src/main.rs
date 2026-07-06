mod args;
mod benchmark_runner;
mod engine;
mod results;
mod validation_engine;

use std::env;
use std::fs::File;
use crate::args::parse_args;
use crate::benchmark_runner::BenchmarkRunner;
use crate::engine::Engine;
use crate::validation_engine::ValidationEngine;

// Usage: rudof_v1 <data_path> <data_format> <shapes_path> \
//          <shapes_format> <stats_path> <report_path> [runs] [warm_up] [timeout] [min_validation_iterations]
//
// - data_path: Path to an RDF file containing the data graph
// - data_format: RDF format of the <data_path>
// - shapes_path: Path to a SHACL shapes file
// - shapes_format: RDF format of the <shapes_path>
// - stats_path: Path to save the stats report file
// - report_path: Path to save the SHACL validation report (Turtle)
// - runs: Number of benchmark runs (Result runs = runs - warm_up)
// - warm_up: Number of runs for warm up
// - timeout: Timeout in seconds for each run
// - min_valid_iterations: Minimum number of valid runs (inclusive) to consider the benchmark successful
fn main() {
    let argv: Vec<String> = env::args().collect();
    let args = parse_args(&argv);
    args.print(Engine::NAME);

    let stats_path = args.stats_path.clone();
    let report_path = args.report_path.clone();

    let results = BenchmarkRunner::new(Engine::new, args).run();

    let stats_file = File::create(&stats_path)
        .expect("Failed to create stats file");
    serde_json::to_writer(stats_file, &results.generate_results())
        .expect("Failed to write stats file");

    println!("[{}] Done -> {}, {}", Engine::NAME, stats_path, report_path);
}
