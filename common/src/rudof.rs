use crate::RdfFormat;

pub trait RudofEngine {
    const DISPLAY_VERSION: &'static str;
    const ID: &'static str;

    fn new() -> Self;
    fn load_data<S: Into<String>>(&mut self, path: S, format: RdfFormat);
    fn load_shapes<S: Into<String>>(&mut self, path: S, format: RdfFormat);
    fn validate(&mut self);
}
