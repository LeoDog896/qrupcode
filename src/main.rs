use clap::{Subcommand, Parser, Args};
use qrcode::QrCode;
use anyhow::Result;
use qrcode::render::unicode;
use std::path::PathBuf;

use std::fs::write;

#[derive(Args, Clone)]
struct BasicOutput {
	/// The content of the QR code
    content: String,

	/// If the QR code should have a quiet zone (padding)
	#[clap(short, long)]
	quiet_zone: bool,

	#[clap(short, long, parse(from_os_str))]
	output: Option<PathBuf>
}

#[derive(Subcommand, Clone)]
enum QROutputType {
	UNICODE(BasicOutput),
	ASCII(BasicOutput),
}

/// A simple QR code generator.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CLI {
    /// The type of output of the QR code.
	#[clap(subcommand)]
	r#type: QROutputType
}
fn main() {
	main_err().unwrap()
}

fn main_err() -> Result<()> {
    let args = CLI::parse();

	let options = match args.r#type.clone() {
		QROutputType::ASCII(options) => options,
		QROutputType::UNICODE(options) => options
	};

    let qr_code = match args.r#type.clone() {
		QROutputType::ASCII(_) => 
			QrCode::new(options.content)?.render()
				.light_color(' ')
				.dark_color('#')
				.quiet_zone(options.quiet_zone)
				.build(),
		QROutputType::UNICODE(_) => 
			QrCode::new(options.content)?.render::<unicode::Dense1x2>()
				.dark_color(unicode::Dense1x2::Light)
				.light_color(unicode::Dense1x2::Dark)
				.quiet_zone(options.quiet_zone)
				.build()
		
	};

	match options.output {
		Some(file) => { write(file, qr_code)?; },
		None => { println!("{}", qr_code); }
	};

	return Ok(());
}