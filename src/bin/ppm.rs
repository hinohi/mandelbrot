use mandelbrot::{pallet::Pallet, utils::cast_color, Mandelbrot, OrthodoxMandelbrot};

fn main() {
    let pallet = Pallet::new(-2.0, -2.0, 4.0, 4.0, 10, 10);
    let loop_max = 100;
    println!("P3");
    println!("{} {}", pallet.pixel_width, pallet.pixel_height);
    println!("255");
    for point in pallet.iter_up_to_down_left_to_right() {
        let mut m = OrthodoxMandelbrot::new();
        let degree = m.diverge(point.z, loop_max);
        if degree == loop_max {
            println!("0 0 0");
        } else {
            let r = 1.0 - (1.0 - degree as f64 / loop_max as f64).powi(9);
            let r = cast_color(r);
            println!("{} 0 0", r);
        }
        if point.x + 1 == pallet.pixel_width {
            eprintln!("{}/{}", point.y + 1, pallet.pixel_width);
        }
    }
}
