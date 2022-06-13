use crate::bmp::Bitmap;
use crate::bmp::BitmapDibHeader;
use crate::bmp::BitmapFileHeader;
use crate::bmp::BmpFile;
use std::fs::File;
use std::io::Result;
use std::io::Write;

pub fn write_to_file(output: &mut File, bmp: &BmpFile) -> Result<()> {
    write_bitmap_file_header(output, &bmp.bitmap_file_header)?;
    write_dib_header(output, &bmp.bitmap_dib_header)?;
    write_bitmap(output, &bmp.bitmap)?;

    Ok(())
}

fn write_bitmap_file_header(output: &mut File, header: &BitmapFileHeader) -> Result<()> {
    let signature_buf: [u8; 2] = header.signature.to_ne_bytes();
    output.write(&signature_buf)?;

    let other_bytes_buf: [u8; 8] = header.other_bytes.to_ne_bytes();
    output.write(&other_bytes_buf)?;

    let offset_buf: [u8; 4] = header.offset.to_ne_bytes();
    output.write(&offset_buf)?;

    Ok(())
}

fn write_dib_header(output: &mut File, dib_header: &BitmapDibHeader) -> Result<()> {
    output.write(&dib_header.raw_data)?;
    Ok(())
}

fn write_bitmap(output: &mut File, bitmap: &Bitmap) -> Result<()> {
    let padding_size = bitmap.width * 3 % 4;
    let mut pixel_iterator = bitmap.pixels.iter();

    let mut padding_buffer = Vec::new();
    if padding_size > 0 {
        for _ in 0..padding_size {
            padding_buffer.push(0);
        }
    }

    for _ in 0..bitmap.height {
        for _ in 0..bitmap.width {
            let pixel = pixel_iterator.next().unwrap();
            let pixel_buf: [u8; 3] = [pixel.blue, pixel.green, pixel.red];
            output.write(&pixel_buf)?;
        }
        if padding_size > 0 {
            output.write(&padding_buffer)?;
        }
    }

    Ok(())
}
