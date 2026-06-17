use rudof::formats::{DataFormat, DataReaderMode, InputSpec, ResultShaclValidationFormat, ShaclFormat, ShaclValidationMode};
use rudof::{Rudof, RudofConfig};
use std::env;
use std::fs::File;
use std::hint::black_box;
use std::io::Write;
use std::time::Instant;

/// Usage: rudof_v2 <data_path> <data_format> <shapes_path> <shapes_format> <csv_path> <report_path> [runs] [warm_up]
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
    let args = env::args().collect::<Vec<String>>();

    let data_path = args.get(1).expect("Missing data graph path");
    let data_format_str = args.get(2).expect("Missing data format").to_lowercase();
    let data_format = match data_format_str.as_str() {
        "turtle" => DataFormat::Turtle,
        "ntriples" => DataFormat::NTriples,
        "rdfxml" => DataFormat::RdfXml,
        "trig" => DataFormat::TriG,
        "n3" => DataFormat::N3,
        "nquads" => DataFormat::NQuads,
        "jsonld" => DataFormat::JsonLd,
        _ => panic!("Not expected format"),
    };
    let shapes_path = args.get(3).expect("Missing shapes graph path");
    let shapes_format_str = args.get(4).expect("Missing shapes format").to_lowercase();
    let shapes_format = match shapes_format_str.as_str() {
        "internal" => ShaclFormat::Internal,
        "turtle" => ShaclFormat::Turtle,
        "ntriples" => ShaclFormat::NTriples,
        "rdfxml" => ShaclFormat::RdfXml,
        "trig" => ShaclFormat::TriG,
        "n3" => ShaclFormat::N3,
        "nquads" => ShaclFormat::NQuads,
        "jsonld" => ShaclFormat::JsonLd,
        _ => panic!("Not expected format"),
    };
    let csv_path = args.get(5).expect("Missing csv report path");
    let report_path = args.get(6).expect("Missing validation report path");
    let runs: usize = args.get(7).and_then(|s| s.parse().ok()).unwrap_or(20);
    let warm_up: usize = args.get(8).and_then(|s| s.parse().ok()).unwrap_or(10);
    let mut result: Vec<String> = Vec::new();

    println!("[rudof_v2] Data:    {} ({})", data_path, data_format_str);
    println!("[rudof_v2] Shapes:  {} ({})", shapes_path, shapes_format_str);
    println!("[rudof_v2] CSV:     {}", csv_path);
    println!("[rudof_v2] Report:  {}", report_path);
    println!("[rudof_v2] Runs:    {}, warm-up: {}", runs, warm_up);

    let mut rudof = Rudof::new(RudofConfig::default());

    for idx in 0..(warm_up + runs) {
        rudof.reset_data()
            .execute();
        rudof.reset_shacl()
            .execute();

        rudof.load_data()
            .with_data(&[InputSpec::path(data_path)])
            .with_data_format(&data_format)
            .with_reader_mode(&DataReaderMode::Strict)
            .with_merge(false)
            .execute()
            .unwrap();

        rudof.load_shacl_shapes()
            .with_shacl_schema(&InputSpec::path(shapes_path))
            .with_shacl_schema_format(&shapes_format)
            .with_reader_mode(&DataReaderMode::Strict)
            .execute()
            .unwrap();

        let start = Instant::now();

        black_box(rudof.validate_shacl()
            .with_shacl_validation_mode(black_box(&ShaclValidationMode::Native))
            .execute()
            .unwrap());

        let elapsed = start.elapsed();

        if idx >= warm_up {
            result.push(format!("{}", elapsed.as_micros() as f64 / 1000.0))
        }
        if warm_up > 0 && idx == warm_up - 1 {
            println!("[rudof_v2] Warm-up complete");
        }
    }

    let mut file = File::create(csv_path).expect(&format!("Unable to create file {csv_path}"));
    result.iter().for_each(|x| {
        file.write((&format!("{x}\n")).as_ref()).expect("Unable to write results to csv");
    });
    file.flush().unwrap();

    let mut report_file = File::create(report_path).expect(&format!("Unable to create file {report_path}"));
    rudof.serialize_shacl_validation_results(&mut report_file)
        .with_result_shacl_validation_format(&ResultShaclValidationFormat::Turtle)
        .execute()
        .expect("Failed to serialize SHACL validation report");
    report_file.flush().unwrap();

    println!("[rudof_v2] Done -> {}, {}", csv_path, report_path);
}
