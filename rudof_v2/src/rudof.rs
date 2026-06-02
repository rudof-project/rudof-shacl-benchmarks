use crate::rdf_format::{cnv_rdf_format, cnv_shacl_format};
use common::{RdfFormat, RudofEngine};
use rudof::formats::{DataReaderMode, InputSpec, ShaclValidationMode};
use rudof::{Rudof, RudofConfig};
use std::hint::black_box;

pub struct RudofV2Engine(Rudof);

impl RudofEngine for RudofV2Engine {
    const DISPLAY_VERSION: &'static str = "rudof v0.3";
    const ID: &'static str = "rudof_v2";

    fn new() -> Self {
        let config = RudofConfig::default();

        Self(Rudof::new(config))
    }

    fn load_data<S: Into<String>>(&mut self, path: S, format: RdfFormat) {
        let format = cnv_rdf_format(format);

        self.0.reset_data()
            .execute();
        self.0.load_data()
            .with_data(&[InputSpec::path(path.into())])
            .with_data_format(&format)
            .with_reader_mode(&DataReaderMode::Strict)
            .with_merge(false)
            .execute()
            .unwrap();
    }

    fn load_shapes<S: Into<String>>(&mut self, path: S, format: RdfFormat) {
        let format = cnv_shacl_format(format);

        self.0.reset_shacl()
            .execute();
        self.0.load_shacl_shapes()
            .with_shacl_schema(&InputSpec::path(path.into()))
            .with_shacl_schema_format(&format)
            .with_reader_mode(&DataReaderMode::Strict)
            .execute()
            .unwrap();
    }

    fn validate(&mut self) {
        black_box(
            self.0.validate_shacl()
                .with_shacl_validation_mode(black_box(&ShaclValidationMode::Native))
                .execute()
                .unwrap()
        );
    }
}
