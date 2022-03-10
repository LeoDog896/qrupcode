use image::codecs::{bmp, farbfeld, gif, ico, jpeg, png, pnm, tga};
use image::{ColorType, ImageEncoder, ImageResult};
use std::io::Write;
use std::io::BufWriter;

pub enum SpecificImageFormat {
    Gif,
    Ico,
    Jpeg,
    Png,
    Pbm,
    Pgm,
    Ppm,
    Pam,
    Farbfeld,
    Bmp,
    Tga
}

// Most variables when no features are supported
pub fn save_img_to_buffer<T: Write>(
    fout: &mut BufWriter<T>,
    buf: &[u8],
    width: u32,
    height: u32,
    color: ColorType,
    format: SpecificImageFormat,
) -> ImageResult<()> {

    match format {
        SpecificImageFormat::Gif => gif::GifEncoder::new(fout).encode(buf, width, height, color),
        SpecificImageFormat::Ico => {
            ico::IcoEncoder::new(fout).write_image(buf, width, height, color)
        }
        SpecificImageFormat::Jpeg => {
            jpeg::JpegEncoder::new(fout).write_image(buf, width, height, color)
        }
        SpecificImageFormat::Png => {
            png::PngEncoder::new(fout).write_image(buf, width, height, color)
        }
        SpecificImageFormat::Pbm => {
            pnm::PnmEncoder::new(fout)
                    .with_subtype(pnm::PNMSubtype::Bitmap(pnm::SampleEncoding::Binary))
                    .write_image(buf, width, height, color)
        }
        SpecificImageFormat::Pgm => {
            pnm::PnmEncoder::new(fout)
                    .with_subtype(pnm::PNMSubtype::Graymap(pnm::SampleEncoding::Binary))
                    .write_image(buf, width, height, color)
        }
        SpecificImageFormat::Ppm => {
            pnm::PnmEncoder::new(fout)
                    .with_subtype(pnm::PNMSubtype::Pixmap(pnm::SampleEncoding::Binary))
                    .write_image(buf, width, height, color)
        },
        SpecificImageFormat::Pam => {
            pnm::PnmEncoder::new(fout).write_image(buf, width, height, color)
        }
        SpecificImageFormat::Farbfeld => {
            farbfeld::FarbfeldEncoder::new(fout).write_image(buf, width, height, color)
        }
        SpecificImageFormat::Bmp => {
            bmp::BmpEncoder::new(fout).write_image(buf, width, height, color)
        }
        SpecificImageFormat::Tga => {
            tga::TgaEncoder::new(fout).write_image(buf, width, height, color)
        }
    }
}
