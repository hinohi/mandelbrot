use num_complex::Complex64;

use crate::Mandelbrot;

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

#[derive(Debug, Default, Copy, Clone)]
pub struct PowerParameterizedMandelbrot {
    z: Complex64,
    pow: f64,
}

impl Mandelbrot for PowerParameterizedMandelbrot {
    fn next(&mut self, c: Complex64) {
        self.z = self.z.powf(self.pow) + c;
    }

    fn is_diverge(&self) -> bool {
        self.z.norm_sqr() > 4.0
    }
}
