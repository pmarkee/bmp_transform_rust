use byteorder::ByteOrder;
use byteorder::LittleEndian;
use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::iter::Iterator;

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

pub fn read_bitmap_file_header(buffer: &[u8; 14]) -> BitmapFileHeader {
    BitmapFileHeader {
        signature: LittleEndian::read_u16(buffer),
        other_bytes: LittleEndian::read_u64(&buffer[2..10]),
        offset: LittleEndian::read_u32(&buffer[10..14]),
    }
}

pub fn read_dib_header(buffer: &[u8; 8]) -> BitmapDibHeader {
    BitmapDibHeader {
        width: LittleEndian::read_u32(buffer),
        height: LittleEndian::read_u32(&buffer[4..]),
    }
}

pub fn read_bitmap(buffer: &Vec<u8>, dib_header: &BitmapDibHeader) -> Bitmap {
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

pub fn read_bmp_file(path: &str) -> Result<BmpFile, Error> {
    let mut file = File::open(&path).unwrap();

    let bitmap_file_header = read_bitmap_file_header(&read_header_data(&mut file)?);
    let bitmap_dib_header = read_dib_header(&read_dib_header_data(&mut file)?);
    let bitmap = read_bitmap(
        &read_bitmap_data(&mut file, &bitmap_file_header)?,
        &bitmap_dib_header,
    );

    Ok(BmpFile {
        bitmap_file_header,
        bitmap_dib_header,
        bitmap,
    })
}

fn read_header_data(file: &mut File) -> Result<[u8; 14], Error> {
    let mut buffer = [0; 14];
    file.by_ref().take(14).read(&mut buffer)?;
    Ok(buffer)
}

fn read_dib_header_data(file: &mut File) -> Result<[u8; 8], Error> {
    let mut buffer = [0; 8];
    file.seek(SeekFrom::Start(0x12))?;
    file.by_ref().take(8).read(&mut buffer)?;
    Ok(buffer)
}

fn read_bitmap_data(file: &mut File, header: &BitmapFileHeader) -> Result<Vec<u8>, Error> {
    file.seek(SeekFrom::Start(header.offset as u64))?;
    let mut buffer: Vec<u8> = Vec::new();
    let bytes_read = file.read_to_end(&mut buffer)?;
    println!("Read {:?} bytes", bytes_read);
    Ok(buffer)
}
