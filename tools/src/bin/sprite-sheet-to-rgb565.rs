use std::fs;

/// side length of one square frame, in pixels
const FRAME_SIZE: u32 = 16;

const FEET_COLOR: u16 = 0x18A4;
const FEET_COLOR_VISIBLE: u16 = 0x8AC6;

fn main() {
    let img = image::open("knight.png")
        .expect("Failed to open image")
        .to_rgb8();

    let (width, height) = img.dimensions();
    assert!(
        width % FRAME_SIZE == 0 && height % FRAME_SIZE == 0,
        "sheet {width}x{height} is not a whole number of {FRAME_SIZE}x{FRAME_SIZE} frames"
    );

    let cols = width / FRAME_SIZE;
    let rows = height / FRAME_SIZE;
    let frame_count = cols * rows;

    // frames are laid out in a grid, so a frame's rows are strided across the
    // sheet. Emit them frame by frame instead, so frame N is the contiguous
    // block at N * FRAME_SIZE * FRAME_SIZE * 2
    let mut data: Vec<u8> = Vec::with_capacity((width * height * 2) as usize);

    for frame in 0..frame_count {
        let start_px_x = (frame % cols) * FRAME_SIZE;
        let start_px_y = (frame / cols) * FRAME_SIZE;

        for y in start_px_y..start_px_y + FRAME_SIZE {
            for x in start_px_x..start_px_x + FRAME_SIZE {
                let pixel = img.get_pixel(x, y);
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];

                // RGB888 -> RGB565
                let mut rgb565: u16 =
                    ((r as u16 & 0xF8) << 8) | ((g as u16 & 0xFC) << 3) | ((b as u16) >> 3);

                // feet share the near-black eye color and vanish on the black
                // display bg, so recolor them brown. Only the bottom frame row
                // is feet, the eyes stay dark
                if rgb565 == FEET_COLOR && y - start_px_y == FRAME_SIZE - 1 {
                    rgb565 = FEET_COLOR_VISIBLE;
                }

                // MSB first
                data.push((rgb565 >> 8) as u8);
                data.push(rgb565 as u8);
            }
        }
    }

    let output_path = "../src/sprite_data.rs";

    let content = format!(
        "pub static SPRITE_DATA: [u8; {}] = {:?};\n",
        data.len(),
        data
    );

    fs::write(output_path, content).expect("Failed to write sprite_data.rs");

    println!(
        "Successfully wrote {} frames ({} bytes) to {}",
        frame_count,
        data.len(),
        output_path
    );
}
