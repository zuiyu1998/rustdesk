pub trait ClipboardTransform {
    fn get_source_format(&self) -> &str;

    fn get_target_format(&self) -> &str;

    fn transform(&self, data: Vec<u8>) -> Vec<u8>;
}
