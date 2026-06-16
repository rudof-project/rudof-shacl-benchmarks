use rudof::{RDFFormat, ReaderMode, Rudof, RudofConfig, ShaclFormat, ShaclValidationMode, ShapesGraphSource};
use std::env;
use std::fs::File;
use std::hint::black_box;
use std::io::Write;
use std::time::Instant;

/// Usage: rudof_v1 <data_path> <data_format> <shapes_path> <shapes_format> <csv_path> [runs] [warm_up]
///
/// - data_path: Path to an RDF file containing the data graph
/// - data_format: RDF format of the <data_path>
/// - shapes_path: Path to a SHACL shapes file
/// - shapes_format: RDF format of the <shapes_path>
/// - csv_path: Path to save the CSV report file
/// - runs: Number of benchmark runs (Result runs = runs - warm_up)
/// - warm_up: Number of runs for warm up
fn main() {
    let args = env::args().collect::<Vec<String>>();

    let data_path = args.get(1).expect("Missing data graph path");
    let data_format = match args.get(2)
        .expect("Missing data format")
        .to_lowercase()
        .as_str() {
        "turtle" => RDFFormat::Turtle,
        "ntriples" => RDFFormat::NTriples,
        "rdfxml" => RDFFormat::RDFXML,
        "trig" => RDFFormat::TriG,
        "n3" => RDFFormat::N3,
        "nquads" => RDFFormat::NQuads,
        "jsonld" => RDFFormat::JsonLd,
        _ => panic!("Not expected format"),
    };
    let shapes_path = args.get(3).expect("Missing shapes graph path");
    let shapes_format = match args.get(4)
        .expect("Missing shapes format")
        .to_lowercase()
        .as_str() {
        "internal" => ShaclFormat::Internal,
        "turtle" => ShaclFormat::Turtle,
        "ntriples" => ShaclFormat::NTriples,
        "rdfxml" => ShaclFormat::RDFXML,
        "trig" => ShaclFormat::TriG,
        "n3" => ShaclFormat::N3,
        "nquads" => ShaclFormat::NQuads,
        "jsonld" => ShaclFormat::JsonLd,
        _ => panic!("Not expected format"),
    };
    let csv_path = args.get(5).expect("Missing csv report path");
    let runs: usize = args.get(6).and_then(|s| s.parse().ok()).unwrap_or(20);
    let warm_up: usize = args.get(7).and_then(|s| s.parse().ok()).unwrap_or(10);
    let mut result: Vec<String> = Vec::new();

    let mut rudof = Rudof::new(&RudofConfig::default_config().unwrap()).unwrap();

    for idx in 0..(warm_up + runs) {
        rudof.reset_data();
        rudof.reset_shacl();
        rudof.read_data(&mut File::open(data_path).unwrap(), "Bench", &data_format, None, &ReaderMode::Strict, false).unwrap();
        rudof.read_shacl(&mut File::open(shapes_path).unwrap(), "Bench", &shapes_format, None, &ReaderMode::Strict).unwrap();

        let start = Instant::now();
        black_box(rudof.validate_shacl(black_box(&ShaclValidationMode::Native), black_box(&ShapesGraphSource::CurrentSchema))).unwrap();
        let elapsed = start.elapsed();

        if idx >= warm_up {
            result.push(format!("{}", elapsed.as_micros() as f64 / 1000.0))
        }
    }

    let mut file = File::create(csv_path).expect(&format!("Unable to create file {csv_path}"));
    result.iter().for_each(|x| {
        file.write((&format!("{x}\n")).as_ref()).expect("Unable to write results to csv");
    });
    file.flush().unwrap();
}
