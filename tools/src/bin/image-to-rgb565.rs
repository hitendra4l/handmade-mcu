use std::fs;

fn main() {
    let img = image::open("image.jpg")
        .expect("Failed to open image")
        .to_rgb8();

    assert_eq!(img.width(), 240);
    assert_eq!(img.height(), 320);

    let mut data: Vec<u8> = Vec::with_capacity(240 * 320 * 2);

    for pixel in img.pixels() {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];

        // RGB888 -> RGB565
        let rgb565: u16 = ((r as u16 & 0xF8) << 8) | ((g as u16 & 0xFC) << 3) | ((b as u16) >> 3);

        // MSB first
        data.push((rgb565 >> 8) as u8);
        data.push(rgb565 as u8);
    }

    let output_path = "../src/image_data.rs";

    let content = format!(
        "pub static IMAGE_DATA: [u8; {}] = {:?};\n",
        data.len(),
        data
    );

    fs::write(output_path, content).expect("Failed to write image_data.rs");

    println!("Successfully wrote {} bytes to {}", data.len(), output_path);
}
