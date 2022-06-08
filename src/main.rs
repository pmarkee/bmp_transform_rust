use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

pub mod bmp;

fn main() {
    let img_path = "/home/pmarkee/learning/rust/bmp_transform_rust/resources/bliss.bmp";
    // TODO handle errors instead of unwrap
    let mut file = File::open(&img_path).unwrap();

    let header_data = read_header_data(&mut file).unwrap();
    let header = bmp::read_bitmap_file_header(&header_data);
    println!("{:?}", header);

    let dib_header_data = read_dib_header_data(&mut file).unwrap();
    let dib_header = bmp::read_dib_header(&dib_header_data);
    println!("{:?}", dib_header);

    let bitmap_data = read_bitmap_data(&mut file, &header).unwrap();
    let bitmap = bmp::read_bitmap(&bitmap_data, &dib_header);
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

fn read_bitmap_data(file: &mut File, header: &bmp::BitmapFileHeader) -> Result<Vec<u8>, Error> {
    file.seek(SeekFrom::Start(header.offset as u64))?;
    let mut buffer: Vec<u8> = Vec::new();
    let bytes_read = file.read_to_end(&mut buffer)?;
    println!("Read {:?} bytes", bytes_read);
    Ok(buffer)
}
