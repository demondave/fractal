extern crate framebuffer;

use framebuffer::{Framebuffer};

struct RenderRange {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64
}

fn main() {
    let mut framebuffer = Framebuffer::new("/dev/fb0").unwrap();
    let w = framebuffer.var_screen_info.xres as usize; 
    let h = framebuffer.var_screen_info.yres as usize;
    let mut frame = vec![0_u8;w*h*4_usize];
    let rr = RenderRange{x_min: -2.5_f64, x_max: 1_f64,y_min: -1_f64,y_max: 1_f64};
    //render_burningship(w, h, 50, &rr, &mut frame);
    //render_mandelbrot(w, h, 50, &rr, &mut frame);
    render(w, h, 50,burningship,&rr, &mut frame);
    let fb_u8: &[u8] = &frame; 
    framebuffer.write_frame(&fb_u8);
}

fn u32_to_bw(v: u64,n: u64) -> [u8;4] {
    let bw = ((v as f64 / n as f64) * 255_f64) as u8;
    return [bw,bw,bw,255_u8]
}

fn render(w: usize,h: usize, max_iter: u64,f: fn(f64,f64,u64) -> u64 ,r: &RenderRange, buffer: &mut Vec<u8>) {
    let mut x = r.x_min;
    let mut y = r.y_min;
    let x_off = ((r.x_min - r.x_max) / w as f64).abs();
    let y_off = ((r.y_min - r.y_max) / h as f64).abs();
    let mut c = 0_u32;
    for a in 0..h {
        x = r.x_min;
        for b in 0..w {
            let iterations = f(x, y,max_iter);
            x += x_off;
            buffer[(c*4) as usize..((c*4)+4) as usize].clone_from_slice(&u32_to_bw(iterations,max_iter)[..]);
            c += 1;
        }
        y += y_off;
    }
}

fn render_burningship(w: usize,h: usize,max_iter: u64,r: &RenderRange,buffer: &mut Vec<u8>) {
    let mut x = r.x_min;
    let mut y = r.y_min;
    let x_off = ((r.x_min - r.x_max) / w as f64).abs();
    let y_off = ((r.y_min - r.y_max) / h as f64).abs();
    let mut c = 0_u32;
    for a in 0..h {
        x = r.x_min;
        for b in 0..w {
            let mut iterations = 0_u64;
            let mut zx = x.clone() as f64;
            let mut zy = y.clone() as f64;
            while zx*zx + zy*zy < 4_f64 && iterations < max_iter {
                let xtemp = (zx*zx + zy*zy + x);
                zy = ((2_f64*zx*zy) + y).abs();
                zx = xtemp;
                iterations += 1;
            }
            x += x_off;
            buffer[(c*4) as usize..((c*4)+4) as usize].clone_from_slice(&u32_to_bw(iterations,max_iter)[..]);
            c += 1;
        }
        y += y_off;
    }
}


fn burningship(x: f64,y: f64,max_iter: u64) -> u64{
    let mut iterations = 0_u64;
    let mut zx = x.clone();
    let mut zy = y.clone();
    while zx*zx + zy*zy < 4_f64 && iterations < max_iter {
        let xtemp = (zx*zx + zy*zy + x);
        zy = ((2_f64*zx*zy) + y).abs();
        zx = xtemp;
        iterations += 1;
    }
    return iterations
}

fn mandelbrot(x0: f64, y0: f64,max_iter: u64) -> u64 {
    let mut n = 0_u64;
    let mut x = 0_f64;
    let mut y = 0_f64;
    let mut x2 = 0_f64;
    let mut y2 = 0_f64;
    while x2 + y2 <= 4_f64 && n < max_iter {
        x = 2_f64 * x * y + y0;
        y = x2 - y2 + x0;
        x2 = x * x;
        y2 = y * y;
        n += 1;
    }
    return n;
}
