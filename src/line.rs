use crate::framebuffer::Framebuffer;

pub fn draw_line(fb: &mut Framebuffer, x0: i32, y0: i32, x1: i32, y1: i32, r: u8, g: u8, b: u8) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;
    let (mut x, mut y) = (x0, y0);

    loop {
        fb.set_pixel(x, y, r, g, b);
        if x == x1 && y == y1 { break; }
        let e2 = 2 * error;
        if e2 >= dy { error += dy; x += sx; }
        if e2 <= dx { error += dx; y += sy; }
    }
}