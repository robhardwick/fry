use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "fry")]
#[command(about = "Deep fry your images")]
#[command(version = "1.0")]
pub struct Args {
    /// Input image file path
    #[arg(short, long, default_value = "test.jpg", value_parser = validate_path_exists)]
    pub input: PathBuf,

    /// Output image file path
    #[arg(short, long, default_value = "test-fry.jpg")]
    pub output: PathBuf,

    /// Number of compression iterations
    #[arg(short = 'n', long, default_value_t = 40, value_parser = clap::value_parser!(u64).range(1..))]
    pub iterations: u64,

    /// JPEG quality (1-100, lower = more artifacts)
    #[arg(short, long, default_value_t = 6, value_parser = clap::value_parser!(u8).range(1..=100))]
    pub quality: u8,

    /// Sharpen strength (higher = more sharpening)
    #[arg(long, default_value_t = 8.0, value_parser = validate_non_negative)]
    pub sharpen: f32,

    /// Saturation multiplier (0.0 = grayscale, 1.0 = normal, 2.0 = double saturation)
    #[arg(long, default_value_t = 2.0, value_parser = validate_non_negative)]
    pub saturate: f32,

    /// Gaussian noise strength (0.0 = none, higher = more noise)
    #[arg(long, default_value_t = 10.0, value_parser = validate_non_negative)]
    pub noise: f32,

    /// Gaussian noise seed
    #[arg(long, default_value_t = 90210)]
    pub seed: u64,

    /// Contrast multiplier (1.0 = normal, higher = more contrast)
    #[arg(long, default_value_t = 2.0, value_parser = validate_non_negative)]
    pub contrast: f32,
}

fn validate_path_exists(s: &str) -> Result<PathBuf, &'static str> {
    let path = PathBuf::from(s);
    if path.exists() {
        Ok(path)
    } else {
        Err("File does not exist")
    }
}

fn validate_non_negative(s: &str) -> Result<f32, &'static str> {
    let value: f32 = s.parse().map_err(|_| "Must be a number")?;
    if value < 0.0 {
        Err("Must be non-negative")
    } else {
        Ok(value)
    }
}
