use crate::rdf_format::{cnv_rdf_format, cnv_shacl_format};
use common::{RdfFormat, RudofEngine};
use rudof::formats::{BackendSpec, DataReaderMode, InputSpec, ShaclValidationMode};
use rudof::{Rudof, RudofConfig};
use std::env;
use std::hint::black_box;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub struct RudofQleverEngine {
    rudof: Rudof,
    endpoint: String,
}

impl RudofQleverEngine {
    pub fn clear_cache(&self) {
        let endpoint = self.endpoint.as_str();
        let mut stream = match TcpStream::connect(endpoint) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[rudof_qlever] clear-cache connect to {endpoint} failed: {e}");
                return;
            }
        };
        let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
        let _ = stream.set_write_timeout(Some(Duration::from_secs(5)));

        let req = format!(
            "GET /?cmd=clear-cache HTTP/1.1\r\nHost: {endpoint}\r\nConnection: close\r\n\r\n",
        );

        if let Err(e) = stream.write_all(req.as_bytes()) {
            eprintln!("[rudof_qlever] clear-cache write failed: {e}");
            return;
        }
        let mut sink = Vec::with_capacity(256);
        let _ = stream.read_to_end(&mut sink);
    }
}

impl RudofEngine for RudofQleverEngine {
    const DISPLAY_VERSION: &'static str = "rudof v2 (qlever)";
    const ID: &'static str = "rudof_qlever";

    fn new() -> Self {
        // TODO - If the config branch is merged, this will be simpler

        let cfg_path = env::var("RUDOF_BENCH_QLEVER_CFG")
            .unwrap_or("qlever_config.toml".to_string());

        let endpoint = env::var("RUDOF_QLEVER_ENDPOINT")
            .unwrap_or("localhost:7001".to_string());

        let config = RudofConfig::from_path(cfg_path);

        if let Ok(config) = config {
            return Self { rudof: Rudof::new(config), endpoint };
        }

        eprintln!("[+] Qlever config file not found, using default");
        Self { rudof: Rudof::new(RudofConfig::default()), endpoint }
    }

    fn load_data<S: Into<String>>(&mut self, path: S, format: RdfFormat) {
        let format = cnv_rdf_format(format);

        self.rudof.reset_data()
            .execute();
        self.rudof.load_data()
            .with_data(&[InputSpec::path(path.into())])
            .with_data_format(&format)
            .with_reader_mode(&DataReaderMode::Strict)
            .with_merge(false)
            .with_backend(BackendSpec::Qlever)
            .execute().unwrap();
    }

    fn load_shapes<S: Into<String>>(&mut self, path: S, format: RdfFormat) {
        let format = cnv_shacl_format(format);

        self.rudof.reset_shacl()
            .execute();
        self.rudof.load_shacl_shapes()
            .with_shacl_schema(&InputSpec::path(path.into()))
            .with_shacl_schema_format(&format)
            .with_reader_mode(&DataReaderMode::Strict)
            .execute().unwrap();
    }

    fn validate(&mut self) {
        black_box(
            self.rudof.validate_shacl()
                .with_shacl_validation_mode(black_box(&ShaclValidationMode::Native))
                .execute()
                .unwrap()
        );
    }
}
