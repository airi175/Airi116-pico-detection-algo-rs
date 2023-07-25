use std::path::PathBuf;

#[macro_export]
macro_rules! model_path {
    ($args:ident, $var:tt, $default:literal) => {
        crate::mo