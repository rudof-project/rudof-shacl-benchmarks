use crate::rdf_format::{cnv_rdf_format, cnv_shacl_format};
use common::{RdfFormat, RudofEngine};
use rudof::formats::{BackendSpec, DataReaderMode, InputSpec, ShaclValidationMode};
use rudof::{Rudof, RudofConfig};
use std::env;
use std::hint::black_box;

pub struct RudofQleverEngine(Rudof);

impl RudofEngine for RudofQleverEngine {
    const DISPLAY_VERSION: &'static str = "rudof v0.3 (qlever)";
    const ID: &'static str = "rudof_qlever";

    fn new() -> Self {
        // TODO - If the config branch is merged, this will be simpler

        let cfg_path = env::var("RUDOF_BENCH_QLEVER_CFG")
            .unwrap_or("qlever_config.toml".to_string());

        let config = RudofConfig::from_path(cfg_path);

        if let Ok(config) = config {
            return Self(Rudof::new(config));
        }

        eprintln!("[+] Config file not found, using default");
        Self(Rudof::new(RudofConfig::default()))
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
            .with_backend(BackendSpec::Qlever)
            .execute().unwrap();
    }

    fn load_shapes<S: Into<String>>(&mut self, path: S, format: RdfFormat) {
        let format = cnv_shacl_format(format);

        self.0.reset_shacl()
            .execute();
        self.0.load_shacl_shapes()
            .with_shacl_schema(&InputSpec::path(path.into()))
            .with_shacl_schema_format(&format)
            .with_reader_mode(&DataReaderMode::Strict)
            .execute().unwrap();
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
