use crate::complex::Complex;
use wasm_bindgen::prelude::*;
mod complex;

#[wasm_bindgen]
pub struct Mandelbrot {
    width: u32,
    height: u32,
    max_iter: u8,
    pixels: Vec<u8>,
}

#[wasm_bindgen]
impl Mandelbrot {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, max_iter: u8) -> Self {
        Self {
            width,
            height,
            max_iter,
            pixels: vec![0; (width * height) as usize],
        }
    }

    #[wasm_bindgen]
    pub fn generate(&mut self, scale: f64, center_x: f64, center_y: f64) -> () {
        for y in 0..self.height {
            for x in 0..self.width {
                let cx = (x as f64 - self.width as f64 / 2.0) / scale + center_x;
                let cy = (y as f64 - self.height as f64 / 2.0) / scale + center_y;
                let mut z = Complex::zero();
                let mut c = Complex { r: cx, i: cy };
                let mut z2 = Complex::zero();
                let mut iterations = 0;

                // while (z2.r + z2.i <= 4.0) && iterations < self.max_iter {
                //     z.i = (z.r + z.r) * z.i + cy;
                //     z.r = z2.r - z2.i + cx;
                //     z2.r = z.r * z.r;
                //     z2.i = z.i * z.i;

                //     iterations += 1;
                // }

                while z.length_no_sqrt() < 4.0 && iterations < self.max_iter {
                    z = (z * z) + c;

                    iterations += 1;
                }

                self.pixels[(y * self.width + x) as usize] = iterations;
            }
        }
        // return (0.0 - self.width as f64 / 2.0) / scale + center_x;

        // POCHODNA
        // let dbail: f64 = 1e6;
        // for y in 0..self.height {
        //     for x in 0..self.width {
        //         let cx = (x as f64 - self.width as f64 / 2.0) / scale + center_x;
        //         let cy = (y as f64 - self.height as f64 / 2.0) / scale + center_y;
        //         let c0 = Complex { r: cx, i: cy };

        //         let mut c = c0;
        //         let mut dc = Complex { r: 1.0, i: 0.0 };
        //         let mut dc_sum = Complex { r: 0.0, i: 0.0 };

        //         let mut iterations = 0;

        //         for n in 1..self.max_iter {
        //             c = c * c + c0;
        //             dc = dc * 2.0 * c + Complex { r: 1.0, i: 0.0 };
        //             dc_sum = dc_sum + dc;

        //             if dc_sum.length_no_sqrt() >= dbail {
        //                 iterations = n;
        //                 break;
        //             }
        //         }

        //         self.pixels[(y * self.width + x) as usize] = iterations;
        //     }
        // }
    }

    #[wasm_bindgen]
    pub fn pixels(&self) -> *const u8 {
        self.pixels.as_ptr()
    }

    #[wasm_bindgen]
    pub fn chuj(&self) -> i32 {
        return self.pixels.iter().map(|&pixel| pixel as u32).sum::<u32>() as i32;
    }
}
