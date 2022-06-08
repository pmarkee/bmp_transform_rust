pub mod bmp;

fn main() {
    let img_path = "/home/pmarkee/learning/rust/bmp_transform_rust/resources/bliss.bmp";

    let bmp_file = bmp::read_bmp_file(&img_path).unwrap();
    println!("{:?}", bmp_file);
}
