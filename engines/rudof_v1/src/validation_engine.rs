pub trait ValidationEngine {
    const NAME: &'static str;
    type Report;

    fn load_data(
        &mut self,
        data_path: &str,
        data_format: &str,
        shapes_path: &str,
        shapes_format: &str,
    );

    fn validate(&mut self) -> Self::Report;

    fn generate_report(&self, result: Self::Report) -> String;
}
