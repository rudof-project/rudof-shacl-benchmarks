use crate::RdfFormat;
use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct BenchmarkConfig {
    #[serde(rename = "icdd", default)]
    pub icdd: IcddBenchmarkConfig,
    #[serde(rename = "era", default)]
    pub era: EraBenchmarkConfig,
    #[serde(rename = "lubm", default)]
    pub lubm: LubmBenchmarkConfig,
}

#[derive(Deserialize)]
pub struct IcddBenchmarkConfig {
    #[serde(rename = "sizes", default = "defaults::icdd_sizes")]
    pub sizes: Vec<String>,
    #[serde(rename = "types", default = "defaults::icdd_types")]
    pub types: Vec<String>,
    #[serde(rename = "path", default = "defaults::icdd_path")]
    pub path: String,
    #[serde(rename = "shapes_format", default = "defaults::icdd_shapes_format")]
    pub shapes_format: RdfFormat,
    #[serde(rename = "data_format", default = "defaults::icdd_data_format")]
    pub data_format: RdfFormat,
    #[serde(rename = "disabled", default = "defaults::disabled")]
    pub disabled: bool,
}

impl Default for IcddBenchmarkConfig {
    fn default() -> Self {
        Self {
            sizes: defaults::icdd_sizes(),
            types: defaults::icdd_types(),
            path: defaults::icdd_path(),
            shapes_format: defaults::icdd_shapes_format(),
            data_format: defaults::icdd_data_format(),
            disabled: defaults::disabled(),
        }
    }
}

#[derive(Deserialize)]
pub struct EraBenchmarkConfig {
    #[serde(rename = "data", default = "defaults::era_data")]
    pub data: Vec<String>,
    #[serde(rename = "shapes", default = "defaults::era_shapes")]
    pub shapes: Vec<String>,
    #[serde(rename = "path", default = "defaults::era_path")]
    pub path: String,
    #[serde(rename = "shapes_format", default = "defaults::era_shapes_format")]
    pub shapes_format: RdfFormat,
    #[serde(rename = "data_format", default = "defaults::era_data_format")]
    pub data_format: RdfFormat,
    #[serde(rename = "disabled", default = "defaults::disabled")]
    pub disabled: bool,
}

impl Default for EraBenchmarkConfig {
    fn default() -> Self {
        Self {
            data: defaults::era_data(),
            shapes: defaults::era_shapes(),
            path: defaults::era_path(),
            shapes_format: defaults::era_shapes_format(),
            data_format: defaults::era_data_format(),
            disabled: defaults::disabled(),
        }
    }
}

#[derive(Deserialize)]
pub struct LubmBenchmarkConfig {
    #[serde(rename = "sizes", default = "defaults::lubm_sizes")]
    pub sizes: Vec<String>,
    #[serde(rename = "path", default = "defaults::lubm_path")]
    pub path: String,
    #[serde(rename = "shapes_format", default = "defaults::lubm_shapes_format")]
    pub shapes_format: RdfFormat,
    #[serde(rename = "data_format", default = "defaults::lubm_data_format")]
    pub data_format: RdfFormat,
    #[serde(rename = "disabled", default = "defaults::disabled")]
    pub disabled: bool,
}

impl Default for LubmBenchmarkConfig {
    fn default() -> Self {
        Self {
            sizes: defaults::lubm_sizes(),
            path: defaults::lubm_path(),
            shapes_format: defaults::lubm_shapes_format(),
            data_format: defaults::lubm_data_format(),
            disabled: defaults::disabled(),
        }
    }
}

mod defaults {
    use super::RdfFormat;

    pub(super) fn icdd_sizes() -> Vec<String> { cnv_list(vec![ "1", "2", "3", "4" ]) }
    pub(super) fn icdd_types() -> Vec<String> { cnv_list(vec![ "binary", "directed1ton", "directedbinary" ]) }
    pub(super) fn icdd_path() -> String { "data/dist/icdd".to_string() }
    pub(super) fn icdd_shapes_format() -> RdfFormat { RdfFormat::Turtle }
    pub(super) fn icdd_data_format() -> RdfFormat { RdfFormat::Turtle }

    pub(super) fn era_data() -> Vec<String> { cnv_list(vec![ "es", "fr", "era" ]) }
    pub(super) fn era_shapes() -> Vec<String> { cnv_list(vec![ "core", "era", "tds" ]) }
    pub(super) fn era_path() -> String { "data/dist/era".to_string() }
    pub(super) fn era_shapes_format() -> RdfFormat { RdfFormat::Turtle }
    pub(super) fn era_data_format() -> RdfFormat { RdfFormat::Turtle }

    pub(super) fn lubm_sizes() -> Vec<String> { cnv_list(vec![ "5", "10", "50", "100", "500" ]) }
    pub(super) fn lubm_path() -> String { "data/dist/lubm".to_string() }
    pub(super) fn lubm_shapes_format() -> RdfFormat { RdfFormat::Turtle }
    pub(super) fn lubm_data_format() -> RdfFormat { RdfFormat::NTriples }

    pub(super) fn disabled() -> bool { false }

    fn cnv_list(l: Vec<&str>) -> Vec<String> {
        l.iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
    }
}
