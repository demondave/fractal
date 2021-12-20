extern crate framebuffer;
extern crate termion;

use std::{thread,time};
use std::io::{stdin,stdout, Write};
use framebuffer::{Framebuffer};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

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
    let max_iterations = 50_u64;
    let mut fractal_function = mandelbrot;
    let mut color_function = u64_to_bw;
    let vertical_off = 0.05_f64;
    let horizontal_off = 0.05_f64;

    let mut frame = vec![0_u8;w*h*4_usize];
    let mut rr = RenderRange{x_min: -3_f64, x_max: 4_f64,y_min: -2_f64,y_max: 2_f64};
    //let rr = RenderRange{x_min: -1.8_f64, x_max: -0.02_f64,y_min: -1.7_f64,y_max: 0.055_f64};
    //render_burningship(w, h, 50, &rr, &mut frame);
    //render_mandelbrot(w, h, 50, &rr, &mut frame);
    while true {
        let input = stdin();
        let mut output = stdout().into_raw_mode().unwrap();
        for c in input.keys() {
            match c.unwrap() {
                Key::Char('q') => break,
                Key::Esc => break,
                Key::Up => {
                    move_rr_vertical(&mut rr, vertical_off*-1_f64);
                    draw(w,h,max_iterations,fractal_function,color_function,&rr,&mut frame, &mut framebuffer)
                },
                Key::Down => {
                    move_rr_vertical(&mut rr, vertical_off);
                    draw(w,h,max_iterations,fractal_function,color_function,&rr,&mut frame, &mut framebuffer)
                },
                Key::Left => {
                    move_rr_horizontal(&mut rr, horizontal_off*-1_f64);
                    draw(w,h,max_iterations,fractal_function,color_function,&rr,&mut frame, &mut framebuffer)
                },
                Key::Right => {
                    move_rr_horizontal(&mut rr, horizontal_off);
                    draw(w,h,max_iterations,fractal_function,color_function,&rr,&mut frame, &mut framebuffer)
                },
                Key::Char('a') => {
                    zoom_rr(&mut rr, 0.1_f64);
                    draw(w,h,max_iterations,fractal_function,color_function,&rr,&mut frame, &mut framebuffer)
                },
                Key::Char('d') => {
                    zoom_rr(&mut rr, -0.1_f64);
                    draw(w,h,max_iterations,fractal_function,color_function,&rr,&mut frame, &mut framebuffer)
                },
                _ => {}
            }
            stdout().flush().unwrap();
        }
        /*
        render(w, h, 50,burningship,u64_to_rgb,&rr, &mut frame);
        let fb_u8: &[u8] = &frame; 
        framebuffer.write_frame(&fb_u8);
        */
    }
}

fn draw(w: usize,h: usize, max_iter: u64,f: fn(f64,f64,u64) -> u64 ,col: fn(u64,u64) -> [u8;4],r: &RenderRange, buffer: &mut Vec<u8>,framebuffer: &mut Framebuffer) {
    render(w, h, max_iter,f,col,r, buffer);
    let fb_u8: &[u8] = &buffer; 
    framebuffer.write_frame(&fb_u8);
}

fn zoom_rr(r: &mut RenderRange, f: f64) {
    let y_scale = (r.y_min - r.y_max).abs() * f;
    let x_scale = (r.x_min - r.x_max).abs() * f;
    r.x_min += x_scale;
    r.x_max -= x_scale;
    r.y_min += y_scale;
    r.y_max -= y_scale;
}

fn move_rr_vertical(r: &mut RenderRange, f: f64) {
    let scale = (r.y_min - r.y_max).abs();
    r.y_min += scale * f;
    r.y_max += scale * f;
}

fn move_rr_horizontal(r: &mut RenderRange, f: f64) {
    let scale = (r.x_min - r.x_max).abs();
    r.x_min += scale * f;
    r.x_max += scale * f;
}


fn u64_to_bw(v: u64,n: u64) -> [u8;4] {
    let bw = ((v as f64 / n as f64) * 255_f64) as u8;
    return [bw,bw,bw,255_u8]
}

fn u64_to_rgb(v: u64,n: u64) -> [u8;4] {
    return [
    (v << 1) as u8,
    (v << 2) as u8,
    (v << 3) as u8,
    255_u8]
}

fn render(w: usize,h: usize, max_iter: u64,f: fn(f64,f64,u64) -> u64 ,col: fn(u64,u64) -> [u8;4],r: &RenderRange, buffer: &mut Vec<u8>) {
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
            buffer[(c*4) as usize..((c*4)+4) as usize].clone_from_slice(&col(iterations,max_iter)[..]);
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
        let xtemp = (zx*zx - zy*zy) - x; // (zx*zx + zy*zy) + x
        zy = (2_f64*zx*zy).abs() + y; // (2_f64*zx*zy).abs() + y;
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
