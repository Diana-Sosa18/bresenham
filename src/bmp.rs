use crate::framebuffer::Framebuffer;
use std::fs::File;
use std::io::Write;

pub fn save_bmp(filename: &str, fb: &Framebuffer) {
    let (w, h) = (fb.width as i32, fb.height as i32);
    let padding = (4 - (w * 3) % 4) % 4;
    let data_size = (w * 3 + padding) * h;
    let file_size = 54 + data_size;

    let mut file = File::create(filename).unwrap();
    file.write_all(b"BM").unwrap();
    file.write_all(&(file_size as u32).to_le_bytes()).unwrap();
    file.write_all(&[0; 4]).unwrap();
    file.write_all(&54u32.to_le_bytes()).unwrap();
    file.write_all(&40u32.to_le_bytes()).unwrap();
    file.write_all(&w.to_le_bytes()).unwrap();
    file.write_all(&h.to_le_bytes()).unwrap();
    file.write_all(&1u16.to_le_bytes()).unwrap();
    file.write_all(&24u16.to_le_bytes()).unwrap();
    file.write_all(&[0; 4]).unwrap();
    file.write_all(&(data_size as u32).to_le_bytes()).unwrap();
    file.write_all(&[0; 16]).unwrap();

    for y in (0..fb.height).rev() {
        for x in 0..fb.width {
            let pos = (y * fb.width + x) * 3;
            file.write_all(&[fb.pixels[pos + 2], fb.pixels[pos + 1], fb.pixels[pos]]).unwrap();
        }
        file.write_all(&vec![0; padding as usize]).unwrap();
    }
}