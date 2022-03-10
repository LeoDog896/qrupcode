use image::codecs::{bmp, farbfeld, gif, ico, jpeg, png, pnm, tga, tiff};
use image::error::ImageFormatHint;
use image::{ColorType, ImageEncoder, ImageError, ImageFormat, ImageResult};
use std::{fs::File, io::BufWriter, path::Path};

// Most variables when no features are supported
pub fn save_buffer_with_format_impl(
    path: &Path,
    buf: &[u8],
    width: u32,
    height: u32,
    color: ColorType,
    format: ImageFormat,
) -> ImageResult<()> {
    let fout = &mut BufWriter::new(File::create(path)?);

    match format {
        image::ImageFormat::Gif => gif::GifEncoder::new(fout).encode(buf, width, height, color),
        image::ImageFormat::Ico => {
            ico::IcoEncoder::new(fout).write_image(buf, width, height, color)
        }
        image::ImageFormat::Jpeg => {
            jpeg::JpegEncoder::new(fout).write_image(buf, width, height, color)
        }
        image::ImageFormat::Png => {
            png::PngEncoder::new(fout).write_image(buf, width, height, color)
        }
        image::ImageFormat::Pnm => {
            let ext = path
                .extension()
                .and_then(|s| s.to_str())
                .map_or("".to_string(), |s| s.to_ascii_lowercase());
            match &*ext {
                "pbm" => pnm::PnmEncoder::new(fout)
                    .with_subtype(pnm::PNMSubtype::Bitmap(pnm::SampleEncoding::Binary))
                    .write_image(buf, width, height, color),
                "pgm" => pnm::PnmEncoder::new(fout)
                    .with_subtype(pnm::PNMSubtype::Graymap(pnm::SampleEncoding::Binary))
                    .write_image(buf, width, height, color),
                "ppm" => pnm::PnmEncoder::new(fout)
                    .with_subtype(pnm::PNMSubtype::Pixmap(pnm::SampleEncoding::Binary))
                    .write_image(buf, width, height, color),
                "pam" => pnm::PnmEncoder::new(fout).write_image(buf, width, height, color),
                _ => Err(ImageError::Unsupported(
                    ImageFormatHint::Exact(format).into(),
                )), // Unsupported Pnm subtype.
            }
        }
        image::ImageFormat::Farbfeld => {
            farbfeld::FarbfeldEncoder::new(fout).write_image(buf, width, height, color)
        }
        image::ImageFormat::Bmp => {
            bmp::BmpEncoder::new(fout).write_image(buf, width, height, color)
        }
        image::ImageFormat::Tiff => {
            tiff::TiffEncoder::new(fout).write_image(buf, width, height, color)
        }
        image::ImageFormat::Tga => {
            tga::TgaEncoder::new(fout).write_image(buf, width, height, color)
        }
        format => Err(ImageError::Unsupported(
            ImageFormatHint::Exact(format).into(),
        )),
    }
}
