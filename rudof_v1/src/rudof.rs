use crate::rdf_format::{cnv_rdf_format, cnv_shacl_format};
use common::{RdfFormat, RudofEngine};
use rudof::{ReaderMode, Rudof, RudofConfig, ShaclValidationMode, ShapesGraphSource};
use std::fs::File;
use std::hint::black_box;

pub struct RudofV1Engine(Rudof);

impl RudofEngine for RudofV1Engine {
    const DISPLAY_VERSION: &'static str = "rudof v0.1";

    fn new() -> Self {
        let config = RudofConfig::default_config().unwrap();
        Self(Rudof::new(&config).unwrap())
    }

    fn load_data<S: Into<String>>(&mut self, path: S, format: RdfFormat) {
        let mut reader = File::open(path.into()).expect("Failed to open data file");

        let format = cnv_rdf_format(format);

        self.0.reset_data();
        self.0.read_data(&mut reader, "Bench", &format, None, &ReaderMode::Strict, false).unwrap();
    }

    fn load_shapes<S: Into<String>>(&mut self, path: S, format: RdfFormat) {
        let mut reader = File::open(path.into()).expect("Failed to open shapes file");

        let format = cnv_shacl_format(format);

        self.0.reset_shacl();
        self.0.read_shacl(&mut reader, "Bench", &format, None, &ReaderMode::Strict).unwrap();
    }

    fn validate(&mut self) {
        black_box(self.0.validate_shacl(
            black_box(&ShaclValidationMode::Native),
            black_box(&ShapesGraphSource::CurrentSchema)
        ).unwrap());
    }
}
