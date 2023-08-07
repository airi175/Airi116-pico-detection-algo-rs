use std::path::PathBuf;

#[macro_export]
macro_rules! model_path {
    ($args:ident, $var:tt, $default:literal) => {
        crate::models::model_path($args.$var.as_ref(), $args.models_dir.as_ref(), $default)
    };
}

#[macro_export]
macro_rules! load_model {
    ($model:ident, $path:expr, $name:literal) => {
        $model::load({
            let file = std::fs::File::open($path).context(format!("Cannot find {} model file.", $name))?;
            std::io::BufReader::new(file)
        }).context(format!("Invalid {} mod