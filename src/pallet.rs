use num_complex::Complex64;

#[derive(Debug, Clone)]
pub struct Pallet {
    x_min: f64,
    y_min: f64,
    pub pixel_width: usize,
    pub pixel_height: usize,
    dx: f64,
    dy: f64,
}

impl Pallet {
    pub fn new(
        x_min: f64,
        y_min: f64,
        width: f64,
        height: f64,
        pixel_width_pow: u8,
        pixel_height_pow: u8,
    ) -> Pallet {
        let pixel_width = (1 << pixel_width_pow) + 1;
        let pixel_height = (1 << pixel_height_pow) + 1;
        Pallet {
            x_min,
            y_min,
            pixel_width,
            pixel_height,
            dx: width / (pixel_width - 1) as f64,
            dy: height / (pixel_height - 1) as f64,
        }
    }

    pub fn point(&self, x: usize, y: usize) -> Point {
        assert!(x < self.pixel_width);
        assert!(y < self.pixel_height);
        let z = Complex64::new(
            self.x_min + self.dx * x as f64,
            self.y_min + self.dy * y as f64,
        );
        Point { z, x, y }
    }

    pub fn iter_up_to_down_left_to_right(&self) -> IterUpToDownLeftToRight<'_> {
        IterUpToDownLeftToRight {
            pallet: self,
            x: 0,
            y: 0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub z: Complex64,
    pub x: usize,
    pub y: usize,
}

pub struct IterUpToDownLeftToRight<'a> {
    pallet: &'a Pallet,
    x: usize,
    y: usize,
}

impl<'a> Iterator for IterUpToDownLeftToRight<'a> {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.y == self.pallet.pixel_height {
            return None;
        }
        let point = self.pallet.point(self.x, self.y);
        if self.x + 1 == self.pallet.pixel_width {
            self.x = 0;
            self.y += 1;
        } else {
            self.x += 1;
        }
        Some(point)
    }
}
