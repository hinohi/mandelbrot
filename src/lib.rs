pub mod pallet;
pub mod utils;

use num_complex::Complex64;

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

#[derive(Debug, Default, Copy, Clone)]
pub struct OrthodoxMandelbrot {
    z: Complex64,
}

impl OrthodoxMandelbrot {
    pub fn new() -> OrthodoxMandelbrot {
        OrthodoxMandelbrot {
            z: Complex64::new(0.0, 0.0),
        }
    }
}

impl Mandelbrot for OrthodoxMandelbrot {
    #[inline]
    fn next(&mut self, c: Complex64) {
        self.z = self.z * self.z + c;
    }

    #[inline]
    fn is_diverge(&self) -> bool {
        self.z.norm_sqr() > 4.0
    }
}
