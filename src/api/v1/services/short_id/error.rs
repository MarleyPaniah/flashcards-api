use std::fmt;

#[derive(Debug)]
pub enum ShortIdGeneratorServiceError {
    RequestError(String),
    ParseError(String),
}

impl fmt::Display for ShortIdGeneratorServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occured while generating a short_id")
    }
}
