use std::fs::File;
use std::hint::black_box;
use std::sync::atomic::{AtomicBool, Ordering};
use rudof::srdf::{BuildRDF, SRDFGraph};
use rudof::{
    RDFFormat, ReaderMode, Rudof, RudofConfig, ShaclFormat, ShaclValidationMode, ShapesGraphSource,
    ValidationReport,
};
use crate::args::{parse_data_format, parse_shapes_format};
use crate::validation_engine::ValidationEngine;

static SIZE_LOGGED: AtomicBool = AtomicBool::new(false);

pub struct Engine {
    rudof: Rudof,
}

impl Engine {
    pub fn new() -> Self {
        let rudof = Rudof::new(&RudofConfig::default_config().unwrap()).unwrap();
        Self { rudof }
    }
}

impl ValidationEngine for Engine {
    const NAME: &'static str = "rudof_v1";
    type Report = ValidationReport;

    fn load_data(
        &mut self,
        data_path: &str,
        data_format: &str,
        shapes_path: &str,
        shapes_format: &str,
    ) {
        self.rudof.reset_data();
        self.rudof.reset_shacl();

        self.rudof
            .read_data(
                &mut File::open(data_path).unwrap(),
                "Bench",
                &parse_data_format(data_format),
                None,
                &ReaderMode::Strict,
                false,
            )
            .unwrap();
        self.rudof
            .read_shacl(
                &mut File::open(shapes_path).unwrap(),
                "Bench",
                &parse_shapes_format(shapes_format),
                None,
                &ReaderMode::Strict,
            )
            .unwrap();

        if !SIZE_LOGGED.swap(true, Ordering::Relaxed) {
            println!("[{}] Data graph size:   TODO", Self::NAME);
            println!("[{}] Shapes graph size: TODO", Self::NAME);
        }
    }

    fn validate(&mut self) -> Self::Report {
        black_box(self.rudof.validate_shacl(
            black_box(&ShaclValidationMode::Native),
            black_box(&ShapesGraphSource::CurrentSchema),
        ))
        .unwrap()
    }

    fn generate_report(&self, result: Self::Report) -> String {
        let mut rdf_writer = SRDFGraph::new();
        result
            .to_rdf(&mut rdf_writer)
            .expect("Failed to convert validation report to RDF");
        let mut buf: Vec<u8> = Vec::new();
        rdf_writer
            .serialize(&RDFFormat::Turtle, &mut buf)
            .expect("Failed to serialize validation report as Turtle");
        String::from_utf8(buf).expect("Report is not valid UTF-8")
    }
}
