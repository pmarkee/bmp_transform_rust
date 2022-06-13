use crate::bmp::Bitmap;
use crate::bmp::BitmapDibHeader;
use crate::bmp::BitmapFileHeader;
use crate::bmp::Pixel;

use byteorder::ByteOrder;
use byteorder::LittleEndian;
use std::iter::Iterator;

pub fn parse_bitmap_file_header(buffer: &[u8; 14]) -> BitmapFileHeader {
    BitmapFileHeader {
        signature: LittleEndian::read_u16(buffer),
        other_bytes: LittleEndian::read_u64(&buffer[2..10]),
        offset: LittleEndian::read_u32(&buffer[10..14]),
    }
}

pub fn parse_dib_header(buffer: &[u8; 8]) -> BitmapDibHeader {
    BitmapDibHeader {
        width: LittleEndian::read_u32(buffer),
        height: LittleEndian::read_u32(&buffer[4..]),
    }
}

pub fn parse_bitmap(buffer: &Vec<u8>, dib_header: &BitmapDibHeader) -> Bitmap {
    let padding_size = dib_header.width * 3 % 4;
    let mut buffer_iterator = buffer.iter();

    let mut pixels: Vec<Pixel> = Vec::new();

    for _ in 0..dib_header.height {
        for _ in 0..dib_header.width {
            // TODO handle errors instead of unwrap
            let blue = buffer_iterator.next().unwrap().clone();
            let green = buffer_iterator.next().unwrap().clone();
            let red = buffer_iterator.next().unwrap().clone();
            pixels.push(Pixel { red, green, blue });
        }
        for _ in 0..padding_size {
            buffer_iterator.next();
        }
    }

    Bitmap {
        width: dib_header.width,
        height: dib_header.height,
        pixels,
    }
}
