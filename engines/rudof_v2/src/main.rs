mod args;
mod benchmark_runner;
mod engine;
mod results;
mod validation_engine;

use rudof::formats::{BackendSpec, DataFormat, DataReaderMode, InputSpec, ResultShaclValidationFormat, ShaclFormat, ShaclValidationMode};
use rudof::{Rudof, RudofConfig};
use std::env;
use std::fs::File;
use std::hint::black_box;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};
use crate::args::parse_args;
use crate::benchmark_runner::BenchmarkRunner;
use crate::engine::Engine;
use crate::validation_engine::ValidationEngine;

// Usage: rudof_v2 <data_path> <data_format> <shapes_path> \
//          <shapes_format> <stats_path> <report_path> [runs] [warm_up] [timeout] [min_validation_iterations]
//
// The backend is selected at runtime via the `RUDOF_BACKEND_QLEVER` env var:
// empty OR 0 OR false -> in-memory (default)
// anything else -> qlever
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
    // let qlever_mode = qlever_mode_enabled();

    let stats_path = args.stats_path.clone();
    let report_path = args.report_path.clone();

    // let mut rudof = init_rudof(qlever_mode);
    // let samples = if qlever_mode {
    //     run_qlever(&mut rudof, &args)
    // } else {
    //     run_in_memory(&mut rudof, &args)
    // };

    let results = BenchmarkRunner::new(Engine::new, args).run();

    let stats_file = File::create(&stats_path)
        .expect("Failed to create stats file");
    serde_json::to_writer(stats_file, &results.generate_results())
        .expect("Failed to write stats file");

    println!("[{}] Done -> {}, {}", Engine::NAME, stats_path, report_path);
}

// fn qlever_mode_enabled() -> bool {
//     !matches!(
//         env::var("RUDOF_BACKEND_QLEVER").ok().as_deref(),
//         None | Some("") | Some("0") | Some("false") | Some("FALSE") | Some("False"),
//     )
// }

// fn init_rudof(qlever_mode: bool) -> Rudof {
//     if !qlever_mode {
//         return Rudof::new(RudofConfig::default());
//     }
//     let cfg_path = env::var("RUDOF_BENCH_QLEVER_CFG")
//         .unwrap_or("qlever_config.toml".to_string());
//     match RudofConfig::from_path(&cfg_path) {
//         Ok(config) => Rudof::new(config),
//         Err(_) => {
//             eprintln!("[rudof_v2] Config file '{}' not found, using default", cfg_path);
//             Rudof::new(RudofConfig::default())
//         }
//     }
// }

// fn run_qlever(rudof: &mut Rudof, args: &Args) -> Vec<String> {
//     let endpoint = env::var("RUDOF_QLEVER_ENDPOINT")
//         .unwrap_or("localhost:7001".to_string());
//
//     rudof.load_data()
//         .with_data(&[InputSpec::path(&args.data_path)])
//         .with_data_format(&args.data_format)
//         .with_reader_mode(&DataReaderMode::Strict)
//         .with_merge(false)
//         .with_backend(BackendSpec::Qlever)
//         .execute()
//         .unwrap();
//
//     rudof.load_shacl_shapes()
//         .with_shacl_schema(&InputSpec::path(&args.shapes_path))
//         .with_shacl_schema_format(&args.shapes_format)
//         .with_reader_mode(&DataReaderMode::Strict)
//         .execute()
//         .unwrap();
//
//     println!("[rudof_qlever] Data graph size: TODO");
//     println!("[rudof_qlever] Shapes graph size: TODO");
//
//     fn_loop(args, rudof, |_, rudof| {
//         clear_qlever_cache(&endpoint);
//         time_validate(rudof, &ShaclValidationMode::Sparql)
//     })
// }

// fn print_header(args: &Args, qlever_mode: bool) {
//     let version = if qlever_mode {
//         "rudof_qlever"
//     } else {
//         "rudof_v2"
//     };
//     println!("[{version}] Data:    {} ({})", args.data_path, args.data_format_str);
//     println!("[{version}] Shapes:  {} ({})", args.shapes_path, args.shapes_format_str);
//     println!("[{version}] CSV:     {}", args.csv_path);
//     println!("[{version}] Report:  {}", args.report_path);
//     println!("[{version}] Backend: {}", if qlever_mode { "qlever" } else { "in-memory" });
//     println!("[{version}] Runs:    {}, warm-up: {}", args.runs, args.warm_up);
// }

// fn clear_qlever_cache(endpoint: &str) {
//     let mut stream = match TcpStream::connect(endpoint) {
//         Ok(s) => s,
//         Err(e) => {
//             eprintln!("[rudof_v2] clear-cache connect to {endpoint} failed: {e}");
//             return;
//         }
//     };
//     let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
//     let _ = stream.set_write_timeout(Some(Duration::from_secs(5)));
//
//     let req = format!(
//         "GET /?cmd=clear-cache HTTP/1.1\r\nHost: {endpoint}\r\nConnection: close\r\n\r\n",
//     );
//
//     if let Err(e) = stream.write_all(req.as_bytes()) {
//         eprintln!("[rudof_v2] clear-cache write failed: {e}");
//         return;
//     }
//     let mut sink = Vec::with_capacity(256);
//     let _ = stream.read_to_end(&mut sink);
// }
