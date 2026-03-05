mod flamegraph;
mod criterion;
mod rudof;

pub use criterion::criterion_config;
pub use flamegraph::FlamegraphProfiler;
pub use rudof::{RdfFormat, Rudof, ShaclFormat};

pub const RUDOF_OLD_ID: &'static str = "rudof v0.1";
pub const RUDOF_NEW_ID: &'static str = "rudof v0.2";