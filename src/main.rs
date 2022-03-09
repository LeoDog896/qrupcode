#![deny(missing_docs)]

//! A CLI wrapper around qrcode as an alternative for qrencode

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use qrcode::render::unicode;
use qrcode::QrCode;
use std::path::PathBuf;

use std::fs::write;

#[derive(Args, Clone)]
struct BasicOutput {
    /// The content of the QR code
    content: String
}

#[derive(Subcommand, Clone)]
enum QROutputType {
    /// Print with unicode (each character = 2 pixels vertically, white & black)
    Unicode(BasicOutput),
    /// Print with ASCII (# and space)
    Ascii(BasicOutput),
}

/// A simple QR code generator.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The type of output of the QR code.
    #[clap(subcommand)]
    r#type: QROutputType,

	/// If the QR code should have a quiet zone (padding)
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
        QROutputType::Ascii(options) | QROutputType::Unicode(options) => options,
    };

    let unrendered_qr_code = QrCode::new(options.content)?;

    let qr_code = match args.r#type {
        QROutputType::Ascii(_) => unrendered_qr_code
            .render()
            .light_color(' ')
            .dark_color('#')
            .quiet_zone(args.quiet_zone)
            .build(),
        QROutputType::Unicode(_) => unrendered_qr_code
            .render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Light)
            .light_color(unicode::Dense1x2::Dark)
            .quiet_zone(args.quiet_zone)
            .build(),
    };

    match args.output {
        Some(file) => {
            write(file, qr_code)?;
        }
        None => {
            println!("{}", qr_code);
        }
    };

    Ok(())
}
