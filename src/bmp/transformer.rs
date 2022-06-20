use crate::bmp::Bitmap;
use crate::bmp::BmpFile;
use crate::bmp::Pixel;

pub fn transform(bmp_file: &BmpFile, mask: [bool; 3]) -> BmpFile {
    let new_pixels: Vec<Pixel> = bmp_file
        .bitmap
        .pixels
        .iter()
        .map(|pixel| transform_pixel(pixel, mask))
        .collect();

    BmpFile {
        bitmap_file_header: bmp_file.bitmap_file_header.clone(),
        bitmap_dib_header: bmp_file.bitmap_dib_header.clone(),
        bitmap: Bitmap {
            width: bmp_file.bitmap.width,
            height: bmp_file.bitmap.height,
            pixels: new_pixels,
        },
    }
}

pub fn transform_pixel(pixel: &Pixel, mask: [bool; 3]) -> Pixel {
    let avg = (pixel.red as u16 + pixel.green as u16 + pixel.blue as u16) / 3;
    Pixel {
        // The average of u8's is also a u8 so casting here is okay
        red: if mask[0] { avg as u8 } else { 0 },
        green: if mask[1] { avg as u8 } else { 0 },
        blue: if mask[2] { avg as u8 } else { 0 },
    }
}
