#[derive(Debug)]
pub enum ArmaLintError {
    ParsingError {
        positives: Vec<String>,
        negatives: Vec<String>,
    },
    InvalidInput(String),
    PreprocessNotRoot,
}