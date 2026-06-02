use common::BenchmarkConfig;
use std::env;
use std::fs::File;
use std::io::Read;

pub fn load_config() -> BenchmarkConfig {
    let cfg_path = env::var("RUDOF_BENCH_CFG")
        .unwrap_or("config.toml".to_string());

    let mut f = File::open(cfg_path)
        .expect("An error occured while opening the config file");
    let mut s = String::new();
    f.read_to_string(&mut s)
        .expect("An error occured while reading the config file");

    let config: BenchmarkConfig = toml::from_str(s.as_str())
        .expect("An error occured while parsing the file");

    config
}