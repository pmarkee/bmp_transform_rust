use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::str::FromStr;

mod file_reader;
mod file_writer;
mod parser;
mod transformer;

pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Clone)]
pub struct BitmapFileHeader {
    pub signature: u16,
    pub other_bytes: u64,
    pub offset: u32,
}

#[derive(Clone, Debug)]
pub struct BitmapDibHeader {
    pub raw_data: Vec<u8>,
    pub header_size: u32,
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

pub fn read_bmp_file(path: &str) -> io::Result<BmpFile> {
    let mut file = File::open(&path).unwrap();

    let bitmap_file_header =
        parser::parse_bitmap_file_header(file_reader::read_header_data(&mut file)?);
    let bitmap_dib_header = parser::parse_dib_header(file_reader::read_dib_header_data(&mut file)?);
    let bitmap = parser::parse_bitmap(
        file_reader::read_bitmap_data(&mut file, bitmap_file_header.offset.clone())?,
        &bitmap_dib_header,
    );

    Ok(BmpFile {
        bitmap_file_header,
        bitmap_dib_header,
        bitmap,
    })
}

pub fn write_bmp_file(path: &str, bmp_file: &BmpFile) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(path)?;
    file_writer::write_to_file(&mut file, bmp_file)
}

pub enum Color {
    Grey,
    Red,
    Green,
    Blue,
    Yellow,
    Violet,
    Cyan,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(input: &str) -> Result<Color, Self::Err> {
        match input {
            "grey" => Ok(Color::Grey),
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            "yellow" => Ok(Color::Yellow),
            "violet" => Ok(Color::Violet),
            "cyan" => Ok(Color::Cyan),
            _ => Err(()),
        }
    }
}

pub fn transform(bmp_file: &BmpFile, color: Color) -> BmpFile {
    let mask = match color {
        Color::Grey => [true, true, true],
        Color::Red => [true, false, false],
        Color::Green => [false, true, false],
        Color::Blue => [false, false, true],
        Color::Yellow => [true, true, false],
        Color::Violet => [true, false, true],
        Color::Cyan => [false, true, true],
    };
    transformer::transform(&bmp_file, mask)
}
