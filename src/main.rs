mod args;
mod error;

use clap::Parser;
use image::{DynamicImage, ImageBuffer, Rgb, codecs::jpeg::JpegEncoder};
use imageproc::{filter::filter3x3, map::map_colors_mut, noise::gaussian_noise_mut};
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Cursor;

use args::Args;
use error::Error;

fn main() -> Result<(), Error> {
    let args = Args::parse();

    println!("🖼️  Loading: {}", args.input.display());
    let mut img = image::open(&args.input)?;

    println!("🔥 Starting fry...");
    let mut rgb_img = img.to_rgb8();

    // Apply noise if enabled
    if args.noise > 0.0 {
        println!("📢 Applying noise (strength: {:.2})", args.noise);
        gaussian_noise_mut(&mut rgb_img, args.noise as f64, 0.0, args.seed);
    }

    // Apply sharpen filter
    if args.sharpen > 0.0 {
        println!("🔪 Applying sharpening (strength: {:.2})", args.sharpen);
        let s = args.sharpen;
        let kernel = [0.0, -s, 0.0, -s, 1.0 + 4.0 * s, -s, 0.0, -s, 0.0];
        rgb_img = filter3x3(&rgb_img, &kernel);
    }

    // Apply saturation adjustment
    if args.saturate != 1.0 {
        println!("🌈 Adjusting saturation (factor: {:.2})", args.saturate);
        filter(&mut rgb_img, |p| {
            // Interpolate between grayscale and original color based on saturation
            let luminance = 0.299 * p[0] + 0.587 * p[1] + 0.114 * p[2];
            p.map(|v| (v - luminance) * args.saturate + luminance)
        });
    }

    // Apply contrast adjustment
    if args.contrast != 1.0 {
        println!("🔆 Adjusting contrast (factor: {:.2})", args.contrast);
        filter(&mut rgb_img, |p| p.map(|v| (v - 0.5) * args.contrast + 0.5));
    }

    img = DynamicImage::ImageRgb8(rgb_img);

    // Create progress bar
    let pb = ProgressBar::new(args.iterations as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "🗜️  Compressing [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
        )?
        .progress_chars("#>-"),
    );
    let mut buffer = Vec::new();
    for i in 1..=args.iterations {
        buffer.clear();

        // Save as JPEG with specific quality to buffer
        {
            let mut cursor = Cursor::new(&mut buffer);
            let encoder = JpegEncoder::new_with_quality(&mut cursor, args.quality);
            img.write_with_encoder(encoder)?;
        }

        // Reload from the compressed buffer
        img = image::load_from_memory(&buffer)?;

        // Update progress bar
        pb.set_position(i);
    }
    pb.finish();

    img.save(&args.output)?;
    println!("💾 Saved: {}", args.output.display());
    println!("🎉 Frying complete!");

    Ok(())
}

fn filter<F: Fn([f32; 3]) -> [f32; 3]>(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, filter_pixel: F) {
    map_colors_mut(img, |p| {
        filter_pixel(p.0.map(|v| v as f32 / 255.0))
            .map(|v| (v.clamp(0.0, 1.0) * 255.0) as u8)
            .into()
    });
}
