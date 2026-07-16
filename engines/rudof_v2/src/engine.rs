use std::fs::File;
use std::hint::black_box;
use std::sync::atomic::{AtomicBool, Ordering};
use rudof::{Rudof, RudofConfig};
use rudof::formats::{DataReaderMode, InputSpec, ResultShaclValidationFormat, ShaclValidationMode};
use crate::args::{parse_data_format, parse_shapes_format};
use crate::validation_engine::ValidationEngine;

static SIZE_LOGGED: AtomicBool = AtomicBool::new(false);

pub struct Engine {
    rudof: Rudof,
}

impl Engine {
    pub fn new() -> Self {
        let rudof = Rudof::new(RudofConfig::default());
        Self { rudof }
    }
}

impl ValidationEngine for Engine {
    const NAME: &'static str = "rudof_v2";
    type Report = ();

    fn load_data(
        &mut self,
        data_path: &str,
        data_format: &str,
        shapes_path: &str,
        shapes_format: &str,
    ) {
        self.rudof
            .reset_data()
            .execute();
        self.rudof
            .reset_shacl()
            .execute();

        self.rudof
            .load_data()
            .with_data(&[InputSpec::path(&data_path)])
            .with_data_format(&parse_data_format(&data_format))
            .with_reader_mode(&DataReaderMode::Strict)
            .with_merge(false)
            .execute()
            .unwrap();

        self.rudof
            .load_shacl_shapes()
            .with_shacl_schema(&InputSpec::path(&shapes_path))
            .with_shacl_schema_format(&parse_shapes_format(&shapes_format))
            .with_reader_mode(&DataReaderMode::Strict)
            .execute()
            .unwrap();

        if !SIZE_LOGGED.swap(true, Ordering::Relaxed) {
            println!("[{}] Data graph size:   TODO", Self::NAME);
            println!("[{}] Shapes graph size: TODO", Self::NAME);
        }
    }

    fn validate(&mut self) -> Self::Report {
        black_box(
            self.rudof
                .validate_shacl()
                .with_shacl_validation_mode(black_box(&ShaclValidationMode::Native))
                .execute()
                .unwrap()
        )
    }

    fn generate_report(&self, _: Self::Report) -> String {
        let mut buf: Vec<u8> = Vec::new();
        self.rudof
            .serialize_shacl_validation_results(&mut buf)
            .with_result_shacl_validation_format(&ResultShaclValidationFormat::Turtle)
            .execute()
            .expect("Failed to serialize SHACL validation report");
        String::from_utf8(buf).expect("Report is not valid UTF-8")
    }
}
