#![deny(missing_docs)]

//! A CLI wrapper around qrcode as an alternative for qrencode

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use either::{Either, Left, Right};
use image::{ImageBuffer, Luma};
use qrcode::QrCode;
use qrcode::{render::unicode, EcLevel};
use std::path::PathBuf;

use std::fs::write;

mod write_image;

#[derive(Args, Clone)]
struct BasicOutput {
    /// The content of the QR code
    content: String,
}

#[derive(Subcommand, Clone)]
enum QROutputType {
    /// Print with unicode (each character = 2 pixels vertically, white & black)
    Unicode(BasicOutput),
    /// Print with ASCII (# and space)
    Ascii(BasicOutput),
    // Print in the image format inferred by the file type.
    Image(BasicOutput),
}

/// A simple QR code generator.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The type of output of the QR code.
    #[clap(subcommand)]
    r#type: QROutputType,

    /// If the QR code should have a quiet zone (padding)
    ///
    /// This adds 3 extra characters of required padding for the QR specifications.
    // This is disabled by default for better output purposes.
    #[clap(short, long, global = true)]
    quiet_zone: bool,

    /// A file output. If none is specified, it will output to stdout.
    #[clap(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
}
fn main() {
    main_err().unwrap();
}

fn main_err() -> Result<()> {
    let args = Cli::parse();

    let options = match args.r#type.clone() {
        QROutputType::Ascii(options)
        | QROutputType::Unicode(options)
        | QROutputType::Image(options) => options,
    };

    let unrendered_qr_code = QrCode::with_error_correction_level(options.content, EcLevel::M)
        .expect("QR code could not be constructed -- data is probably too long.");

    let qr_code: Either<String, ImageBuffer<Luma<u8>, Vec<u8>>> = match args.r#type {
        QROutputType::Ascii(_) => Left(
            unrendered_qr_code
                .render()
                .light_color(' ')
                .dark_color('#')
                .quiet_zone(args.quiet_zone)
                .build(),
        ),
        QROutputType::Unicode(_) => Left(
            unrendered_qr_code
                .render::<unicode::Dense1x2>()
                .dark_color(unicode::Dense1x2::Light)
                .light_color(unicode::Dense1x2::Dark)
                .quiet_zone(args.quiet_zone)
                .build(),
        ),
        QROutputType::Image(_) => Right(
            unrendered_qr_code
                .render::<Luma<u8>>()
                .quiet_zone(args.quiet_zone)
                .build(),
        ),
    };

    match args.output {
        Some(file) => {
            if let Right(image) = qr_code {
                image.save(file.to_str().unwrap())?;

                return Ok(());
            }

            write(&file, qr_code.unwrap_left())
                .expect(&format!("Could not write the QR code to file {:?}", file));
        }
        None => {
            println!(
                "{}",
                qr_code.expect_left("An output must be specified for images!")
            );
        }
    };

    Ok(())
}
