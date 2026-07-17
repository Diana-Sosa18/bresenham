mod bmp;
mod framebuffer;
mod line;

use bmp::save_bmp;
use framebuffer::Framebuffer;
use line::draw_line;

fn main() {
    let mut fb = Framebuffer::new(400, 400);

    draw_line(&mut fb, 20, 20, 380, 20, 255, 255, 255);  
    draw_line(&mut fb, 20, 20, 20, 380, 255, 255, 255);  
    draw_line(&mut fb, 20, 20, 380, 380, 255, 255, 255); 
    draw_line(&mut fb, 20, 380, 380, 20, 255, 255, 255); 

    draw_line(&mut fb, 200, 200, 350, 250, 255, 0, 0);
    draw_line(&mut fb, 200, 200, 250, 350, 255, 0, 0); 

    save_bmp("out.bmp", &fb);
    println!("out.bmp generado");
}
