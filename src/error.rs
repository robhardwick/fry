use std::fmt;

#[derive(Debug)]
pub enum Error {
    Image(image::ImageError),
    ProgressBar(indicatif::style::TemplateError),
    InputFileNotFound(std::path::PathBuf),
    InvalidQuality(u8),
}

impl From<image::ImageError> for Error {
    fn from(err: image::ImageError) -> Self {
        Error::Image(err)
    }
}

impl From<indicatif::style::TemplateError> for Error {
    fn from(err: indicatif::style::TemplateError) -> Self {
        Error::ProgressBar(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Image(err) => write!(f, "Image error: {}", err),
            Error::ProgressBar(err) => write!(f, "Progress bar error: {}", err),
            Error::InputFileNotFound(path) => {
                write!(f, "Input file not found: {}", path.display())
            }
            Error::InvalidQuality(q) => {
                write!(f, "Invalid quality: {}. Must be between 1 and 100.", q)
            }
        }
    }
}
