use std::fs::File;
use std::io::Error;

mod bmp_parser;
mod file_reader;

// TODO rethink what should be public and private here
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct BitmapFileHeader {
    pub signature: u16,
    pub other_bytes: u64,
    pub offset: u32,
}

#[derive(Debug)]
pub struct BitmapDibHeader {
    pub width: u32,
    pub height: u32,
}

pub struct Bitmap {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Pixel>,
}

pub struct BmpFile {
    pub bitmap_file_header: BitmapFileHeader,
    pub bitmap_dib_header: BitmapDibHeader,
    pub bitmap: Bitmap,
}

impl std::fmt::Debug for BitmapFileHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "BitmapFileHeader {{ signature: {:#x}, offset: {:#x} }}",
            self.signature, self.offset
        )
    }
}

impl std::fmt::Debug for BmpFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("BmpFile")
            .field("bitmap_file_header", &self.bitmap_file_header)
            .field("bitmap_dib_header", &self.bitmap_dib_header)
            .finish()
    }
}

pub fn read_bmp_file(path: &str) -> Result<BmpFile, Error> {
    let mut file = File::open(&path).unwrap();

    let bitmap_file_header =
        bmp_parser::parse_bitmap_file_header(&file_reader::read_header_data(&mut file)?);
    let bitmap_dib_header =
        bmp_parser::parse_dib_header(&file_reader::read_dib_header_data(&mut file)?);
    let bitmap = bmp_parser::parse_bitmap(
        &file_reader::read_bitmap_data(&mut file, bitmap_file_header.offset.clone())?,
        &bitmap_dib_header,
    );

    Ok(BmpFile {
        bitmap_file_header,
        bitmap_dib_header,
        bitmap,
    })
}
