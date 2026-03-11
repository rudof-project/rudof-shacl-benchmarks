use rudof_lib::{RDFFormat, ReaderMode, Rudof, RudofConfig, ShaclFormat, ShaclValidationMode, ShapesGraphSource};
use std::env;
use std::fs::File;
use std::hint::black_box;
use std::io::Write;
use std::time::Instant;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let data_path = args.get(1).expect("Missing data graph path");
    let data_format = match args.get(2)
        .expect("Missing data format")
        .to_lowercase()
        .as_str() {
        "turtle" => RDFFormat::Turtle,
        "ntriples" => RDFFormat::NTriples,
        "rdfxml" => RDFFormat::Rdfxml,
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
        "rdfxml" => ShaclFormat::RdfXml,
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
        rudof.read_data(&mut File::open(data_path).unwrap(), "Bench", Some(&data_format), None, Some(&ReaderMode::Strict), Some(false)).unwrap();
        rudof.read_shacl(&mut File::open(shapes_path).unwrap(), "Bench", Some(&shapes_format), None, Some(&ReaderMode::Strict)).unwrap();

        let start = Instant::now();
        black_box(rudof.validate_shacl(black_box(Some(&ShaclValidationMode::Native)), black_box(Some(&ShapesGraphSource::CurrentSchema)))).unwrap();
        let elapsed = start.elapsed();

        if idx >= warm_up {
            result.push(format!("{}", elapsed.as_millis()))
        }
    }

    let mut file = File::create(csv_path).expect(&format!("Unable to create file {csv_path}"));
    result.iter().for_each(|x| {
        file.write((&format!("{x}\n")).as_ref()).expect("Unable to write results to csv");
    });
    file.flush().unwrap();
}
