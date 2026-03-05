use std::fs::File;

pub enum Rudof {
    Old(rudof_old::Rudof),
    New(rudof_new::Rudof),
}

pub enum RdfFormat {
    Old(rudof_old::RDFFormat),
    New(rudof_new::RDFFormat),
}

pub enum ShaclFormat {
    Old(rudof_old::ShaclFormat),
    New(rudof_new::ShaclFormat),
}

impl Rudof {
    pub fn load_data(&mut self, data_path: &str, format: &RdfFormat) {
        let mut reader = File::open(data_path).expect("Failed to open data file");

        match self {
            Rudof::Old(rudof) => {
                let RdfFormat::Old(format) = format else { unreachable!() };

                rudof.reset_data();
                rudof.read_data(&mut reader, "Bench", format, None, &rudof_old::ReaderMode::Strict, false).unwrap();
            },
            Rudof::New(rudof) => {
                let RdfFormat::New(format) = format else { unreachable!() };

                rudof.reset_data();
                rudof.read_data(&mut reader, "Bench", Some(format), None, Some(&rudof_new::ReaderMode::Strict), Some(false)).unwrap();
            }
        }
    }

    pub fn load_shapes(&mut self, shapes_path: &str, format: &ShaclFormat) {
        let mut reader = File::open(shapes_path).expect("Failed to open shapes file");

        match self {
            Rudof::Old(rudof) => {
                let ShaclFormat::Old(format) = format else { unreachable!() };

                rudof.reset_shacl();
                rudof.read_shacl(&mut reader, "Bench", format, None, &rudof_old::ReaderMode::Strict).unwrap()
            },
            Rudof::New(rudof) => {
                let ShaclFormat::New(format) = format else { unreachable!() };

                rudof.reset_shacl();
                rudof.read_shacl(&mut reader, "Bench", Some(format), None, Some(&rudof_new::ReaderMode::Strict)).unwrap()
            }
        }
    }

    pub fn default_old() -> Self {
        let config = rudof_old::RudofConfig::default_config().unwrap();

        Rudof::Old(rudof_old::Rudof::new(&config).unwrap())
    }

    pub fn default_new() -> Self {
        let config = rudof_new::RudofConfig::default_config().unwrap();

        Rudof::New(rudof_new::Rudof::new(&config).unwrap())
    }
}
