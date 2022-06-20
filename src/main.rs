pub mod bmp;

fn main() {
    let img_path = "/home/pmarkee/learning/rust/bmp_transform_rust/resources/bliss.bmp";
    let output_path = "/home/pmarkee/learning/rust/bmp_transform_rust/resources/out.bmp";

    let bmp_file = bmp::read_bmp_file(&img_path).unwrap();
    let transformed = bmp::transform_to_greyscale(&bmp_file, bmp::Color::Grey);
    bmp::write_bmp_file(output_path, &transformed).unwrap();
}
