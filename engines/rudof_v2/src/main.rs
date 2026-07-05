use rudof::formats::{BackendSpec, DataFormat, DataReaderMode, InputSpec, ResultShaclValidationFormat, ShaclFormat, ShaclValidationMode};
use rudof::{Rudof, RudofConfig};
use std::env;
use std::fs::File;
use std::hint::black_box;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

/// Usage: rudof_v2 <data_path> <data_format> <shapes_path> <shapes_format> <csv_path> <report_path> [runs] [warm_up]
///
/// The backend is selected at runtime via the `RUDOF_BACKEND_QLEVER` env var:
/// empty OR 0 OR false -> in-memory (default)
/// anything else -> qlever
///
/// - data_path: Path to an RDF file containing the data graph
/// - data_format: RDF format of the <data_path>
/// - shapes_path: Path to a SHACL shapes file
/// - shapes_format: RDF format of the <shapes_path>
/// - csv_path: Path to save the CSV report file
/// - report_path: Path to save the SHACL validation report (Turtle)
/// - runs: Number of benchmark runs (Result runs = runs - warm_up)
/// - warm_up: Number of runs for warm up
fn main() {
    let args = Args::from_env();
    let qlever_mode = qlever_mode_enabled();

    print_header(&args, qlever_mode);

    let mut rudof = init_rudof(qlever_mode);

    let samples = if qlever_mode {
        run_qlever(&mut rudof, &args)
    } else {
        run_in_memory(&mut rudof, &args)
    };

    write_csv(&args.csv_path, &samples);

    println!("[rudof_v2] Done -> {}, {}", args.csv_path, args.report_path);
}

struct Args {
    data_path: String,
    data_format: DataFormat,
    data_format_str: String,
    shapes_path: String,
    shapes_format: ShaclFormat,
    shapes_format_str: String,
    csv_path: String,
    report_path: String,
    runs: usize,
    warm_up: usize,
}

impl Args {
    fn from_env() -> Self {
        let args = env::args().collect::<Vec<String>>();

        let data_path = args.get(1).expect("Missing data graph path").clone();
        let data_format_str = args.get(2).expect("Missing data format").to_lowercase();
        let data_format = parse_data_format(&data_format_str);
        let shapes_path = args.get(3).expect("Missing shapes graph path").clone();
        let shapes_format_str = args.get(4).expect("Missing shapes format").to_lowercase();
        let shapes_format = parse_shapes_format(&shapes_format_str);
        let csv_path = args.get(5).expect("Missing csv report path").clone();
        let report_path = args.get(6).expect("Missing validation report path").clone();
        let runs: usize = args.get(7).and_then(|s| s.parse().ok()).unwrap_or(20);
        let warm_up: usize = args.get(8).and_then(|s| s.parse().ok()).unwrap_or(10);

        Self {
            data_path,
            data_format,
            data_format_str,
            shapes_path,
            shapes_format,
            shapes_format_str,
            csv_path,
            report_path,
            runs,
            warm_up,
        }
    }
}

fn parse_data_format(s: &str) -> DataFormat {
    match s {
        "turtle" => DataFormat::Turtle,
        "ntriples" => DataFormat::NTriples,
        "rdfxml" => DataFormat::RdfXml,
        "trig" => DataFormat::TriG,
        "n3" => DataFormat::N3,
        "nquads" => DataFormat::NQuads,
        "jsonld" => DataFormat::JsonLd,
        _ => panic!("Not expected format"),
    }
}

fn parse_shapes_format(s: &str) -> ShaclFormat {
    match s {
        "internal" => ShaclFormat::Internal,
        "turtle" => ShaclFormat::Turtle,
        "ntriples" => ShaclFormat::NTriples,
        "rdfxml" => ShaclFormat::RdfXml,
        "trig" => ShaclFormat::TriG,
        "n3" => ShaclFormat::N3,
        "nquads" => ShaclFormat::NQuads,
        "jsonld" => ShaclFormat::JsonLd,
        _ => panic!("Not expected format"),
    }
}

fn qlever_mode_enabled() -> bool {
    !matches!(
        env::var("RUDOF_BACKEND_QLEVER").ok().as_deref(),
        None | Some("") | Some("0") | Some("false") | Some("FALSE") | Some("False"),
    )
}

fn init_rudof(qlever_mode: bool) -> Rudof {
    if !qlever_mode {
        return Rudof::new(RudofConfig::default());
    }
    let cfg_path = env::var("RUDOF_BENCH_QLEVER_CFG")
        .unwrap_or("qlever_config.toml".to_string());
    match RudofConfig::from_path(&cfg_path) {
        Ok(config) => Rudof::new(config),
        Err(_) => {
            eprintln!("[rudof_v2] Config file '{}' not found, using default", cfg_path);
            Rudof::new(RudofConfig::default())
        }
    }
}

fn print_header(args: &Args, qlever_mode: bool) {
    let version = if qlever_mode {
        "rudof_qlever"
    } else {
        "rudof_v2"
    };
    println!("[{version}] Data:    {} ({})", args.data_path, args.data_format_str);
    println!("[{version}] Shapes:  {} ({})", args.shapes_path, args.shapes_format_str);
    println!("[{version}] CSV:     {}", args.csv_path);
    println!("[{version}] Report:  {}", args.report_path);
    println!("[{version}] Backend: {}", if qlever_mode { "qlever" } else { "in-memory" });
    println!("[{version}] Runs:    {}, warm-up: {}", args.runs, args.warm_up);
}

fn run_qlever(rudof: &mut Rudof, args: &Args) -> Vec<String> {
    let endpoint = env::var("RUDOF_QLEVER_ENDPOINT")
        .unwrap_or("localhost:7001".to_string());

    rudof.load_data()
        .with_data(&[InputSpec::path(&args.data_path)])
        .with_data_format(&args.data_format)
        .with_reader_mode(&DataReaderMode::Strict)
        .with_merge(false)
        .with_backend(BackendSpec::Qlever)
        .execute()
        .unwrap();

    rudof.load_shacl_shapes()
        .with_shacl_schema(&InputSpec::path(&args.shapes_path))
        .with_shacl_schema_format(&args.shapes_format)
        .with_reader_mode(&DataReaderMode::Strict)
        .execute()
        .unwrap();

    println!("[rudof_qlever] Data graph size: TODO");
    println!("[rudof_qlever] Shapes graph size: TODO");

    fn_loop(args, rudof, |_, rudof| {
        clear_qlever_cache(&endpoint);
        time_validate(rudof, &ShaclValidationMode::Sparql)
    })
}

fn run_in_memory(rudof: &mut Rudof, args: &Args) -> Vec<String> {
    fn_loop(args, rudof, |idx, rudof| {
        rudof.reset_data().execute();
        rudof.reset_shacl().execute();

        rudof.load_data()
            .with_data(&[InputSpec::path(&args.data_path)])
            .with_data_format(&args.data_format)
            .with_reader_mode(&DataReaderMode::Strict)
            .with_merge(false)
            .execute()
            .unwrap();

        rudof.load_shacl_shapes()
            .with_shacl_schema(&InputSpec::path(&args.shapes_path))
            .with_shacl_schema_format(&args.shapes_format)
            .with_reader_mode(&DataReaderMode::Strict)
            .execute()
            .unwrap();

        if idx == 0 {
            println!("[rudof_v2] Data graph size: TODO");
            println!("[rudof_v2] Shapes graph size: TODO")
        }

        time_validate(rudof, &ShaclValidationMode::Native)
    })
}

fn fn_loop<F>(args: &Args, rudof: &mut Rudof, mut measure: F) -> Vec<String>
where
    F: FnMut(usize, &mut Rudof) -> u128,
{
    let mut samples = Vec::with_capacity(args.runs);
    for idx in 0..(args.warm_up + args.runs) {
        let micros = measure(idx, rudof);
        if idx >= args.warm_up {
            samples.push(format!("{}", micros as f64 / 1000.0));

            write_report(rudof, &args.report_path)
        }
        if args.warm_up > 0 && idx == args.warm_up - 1 {
            println!("[rudof_v2] Warm-up complete");
        }
    }
    samples
}

fn time_validate(rudof: &mut Rudof, mode: &ShaclValidationMode) -> u128 {
    let start = Instant::now();
    black_box(rudof.validate_shacl()
        .with_shacl_validation_mode(black_box(mode))
        .execute()
        .unwrap());
    start.elapsed().as_micros()
}

fn write_csv(path: &str, samples: &[String]) {
    let mut file = File::create(path).expect(&format!("Unable to create file {path}"));
    for line in samples {
        file.write_all(format!("{line}\n").as_bytes())
            .expect("Unable to write results to csv");
    }
    file.flush().unwrap();
}

fn write_report(rudof: &mut Rudof, path: &str) {
    let mut file = File::create(path).expect(&format!("Unable to create file {path}"));
    rudof.serialize_shacl_validation_results(&mut file)
        .with_result_shacl_validation_format(&ResultShaclValidationFormat::Turtle)
        .execute()
        .expect("Failed to serialize SHACL validation report");
    file.flush().unwrap();
}

fn clear_qlever_cache(endpoint: &str) {
    let mut stream = match TcpStream::connect(endpoint) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[rudof_v2] clear-cache connect to {endpoint} failed: {e}");
            return;
        }
    };
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(5)));

    let req = format!(
        "GET /?cmd=clear-cache HTTP/1.1\r\nHost: {endpoint}\r\nConnection: close\r\n\r\n",
    );

    if let Err(e) = stream.write_all(req.as_bytes()) {
        eprintln!("[rudof_v2] clear-cache write failed: {e}");
        return;
    }
    let mut sink = Vec::with_capacity(256);
    let _ = stream.read_to_end(&mut sink);
}
