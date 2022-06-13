use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::io::Seek;
use std::io::SeekFrom;

pub fn read_header_data(file: &mut File) -> Result<[u8; 14]> {
    let mut buffer = [0; 14];
    file.by_ref().take(14).read(&mut buffer)?;
    Ok(buffer)
}

pub fn read_dib_header_data(file: &mut File) -> Result<[u8; 8]> {
    let mut buffer = [0; 8];
    file.seek(SeekFrom::Start(0x12))?;
    file.by_ref().take(8).read(&mut buffer)?;
    Ok(buffer)
}

pub fn read_bitmap_data(file: &mut File, offset: u32) -> Result<Vec<u8>> {
    file.seek(SeekFrom::Start(offset as u64))?;
    let mut buffer: Vec<u8> = Vec::new();
    let bytes_read = file.read_to_end(&mut buffer)?;
    println!("Read {:?} bytes", bytes_read);
    Ok(buffer)
}
