use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::io::Seek;
use std::io::SeekFrom;

use byteorder::ByteOrder;
use byteorder::LittleEndian;

pub fn read_header_data(file: &mut File) -> Result<[u8; 14]> {
    let mut buffer = [0; 14];
    file.by_ref().take(14).read(&mut buffer)?;
    Ok(buffer)
}

pub fn read_dib_header_data(file: &mut File) -> Result<Vec<u8>> {
    file.seek(SeekFrom::Start(0x0e))?;
    let mut dib_header_size_buf = [0; 4];
    file.by_ref().take(4).read(&mut dib_header_size_buf)?;
    let dib_header_size = LittleEndian::read_u32(&dib_header_size_buf);

    file.seek(SeekFrom::Start(0x0e))?;
    let mut buffer = vec![0; dib_header_size as usize];
    file.by_ref().read_exact(&mut buffer)?;
    Ok(buffer)
}

pub fn read_bitmap_data(file: &mut File, offset: u32) -> Result<Vec<u8>> {
    file.seek(SeekFrom::Start(offset as u64))?;
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
