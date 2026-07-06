use rudof::{RDFFormat, ShaclFormat};

#[derive(Clone)]
pub struct Args {
    pub data_path: String,
    pub data_format: String,

    pub shapes_path: String,
    pub shapes_format: String,

    pub stats_path: String,
    pub report_path: String,

    pub runs: usize,
    pub warm_up: usize,
    pub timeout: u64,
    pub min_valid_iter: usize,
}

impl Args {
    pub fn print(&self, name: &str) {
        println!("[{name}] Data:               {} ({})", self.data_path, self.data_format);
        println!("[{name}] Shapes:             {} ({})", self.shapes_path, self.shapes_format);
        println!("[{name}] Stats:              {}", self.stats_path);
        println!("[{name}] Report:             {}", self.report_path);
        println!("[{name}] Runs:               {}", self.runs);
        println!("[{name}] Warm-up:            {}", self.warm_up);
        println!("[{name}] Timeout:            {} s", self.timeout);
        println!("[{name}] Minimum valid runs: {} (inclusive)", self.min_valid_iter);
    }
}

pub fn parse_args(argv: &[String]) -> Args {
    fn get(argv: &[String], idx: usize, msg: &str, default: Option<&str>) -> String {
        argv.get(idx)
            .map(String::as_str)
            .or(default)
            .unwrap_or_else(|| panic!("{msg}"))
            .to_string()
    }

    Args {
        data_path: get(argv, 1, "Missing data graph path", None),
        data_format: get(argv, 2, "Missing data format", None),
        shapes_path: get(argv, 3, "Missing shapes graph path", None),
        shapes_format: get(argv, 4, "Missing shapes format", None),
        stats_path: get(argv, 5, "Missing stats report path", None),
        report_path: get(argv, 6, "Missing validation report path", None),
        runs: get(argv, 7, "", Some("20")).parse().expect("Invalid runs value"),
        warm_up: get(argv, 8, "", Some("10")).parse().expect("Invalid warm_up value"),
        timeout: get(argv, 9, "", Some("300")).parse().expect("Invalid timeout value"),
        min_valid_iter: get(argv, 10, "", Some("8")).parse().expect("Invalid min_valid_iter value"),
    }
}

pub fn parse_data_format(s: &str) -> RDFFormat {
    match s.to_lowercase().as_str() {
        "turtle" => RDFFormat::Turtle,
        "ntriples" => RDFFormat::NTriples,
        "rdfxml" => RDFFormat::RDFXML,
        "trig" => RDFFormat::TriG,
        "n3" => RDFFormat::N3,
        "nquads" => RDFFormat::NQuads,
        "jsonld" => RDFFormat::JsonLd,
        other => panic!("Not expected data format: {other}"),
    }
}

pub fn parse_shapes_format(s: &str) -> ShaclFormat {
    match s.to_lowercase().as_str() {
        "internal" => ShaclFormat::Internal,
        "turtle" => ShaclFormat::Turtle,
        "ntriples" => ShaclFormat::NTriples,
        "rdfxml" => ShaclFormat::RDFXML,
        "trig" => ShaclFormat::TriG,
        "n3" => ShaclFormat::N3,
        "nquads" => ShaclFormat::NQuads,
        "jsonld" => ShaclFormat::JsonLd,
        other => panic!("Not expected shapes format: {other}"),
    }
}
