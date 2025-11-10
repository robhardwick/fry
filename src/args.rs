use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "fry")]
#[command(about = "Deep fry your images")]
#[command(version = "1.0")]
pub struct Args {
    /// Input image file path
    #[arg(short, long, default_value = "test.jpg")]
    pub input: PathBuf,

    /// Output image file path
    #[arg(short, long, default_value = "test-fry.jpg")]
    pub output: PathBuf,

    /// Number of compression iterations
    #[arg(short = 'n', long, default_value_t = 40)]
    pub iterations: u64,

    /// JPEG quality (1-100, lower = more artifacts)
    #[arg(short, long, default_value_t = 6)]
    pub quality: u8,

    /// Sharpen strength (higher = more sharpening)
    #[arg(long, default_value_t = 8.0)]
    pub sharpen: f32,

    /// Saturation multiplier (0.0 = grayscale, 1.0 = normal, 2.0 = double saturation)
    #[arg(long, default_value_t = 2.0)]
    pub saturate: f32,

    /// Gaussian noise strength (0.0 = none, higher = more noise)
    #[arg(long, default_value_t = 10.0)]
    pub noise: f64,

    /// Gaussian noise seed
    #[arg(long, default_value_t = 90210)]
    pub seed: u64,

    /// Contrast multiplier (1.0 = normal, higher = more contrast)
    #[arg(long, default_value_t = 2.0)]
    pub contrast: f32,
}
