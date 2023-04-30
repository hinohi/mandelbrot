mod impls;
pub mod pallet;
pub mod utils;

use num_complex::Complex64;

pub use crate::impls::*;

pub trait Mandelbrot {
    fn next(&mut self, c: Complex64);

    fn is_diverge(&self) -> bool;

    fn diverge(&mut self, c: Complex64, loop_max: u32) -> u32 {
        for i in 0..loop_max {
            self.next(c);
            if self.is_diverge() {
                return i;
            }
        }
        loop_max
    }
}
