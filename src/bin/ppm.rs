use clap::{AppSettings, Clap};

use mandelbrot::{orthodox::OrthodoxMandelbrot, pallet::Pallet, utils::cast_color, Mandelbrot};

#[derive(Debug, Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long, default_value = "10")]
    pixel_pow: u8,
    #[clap(short, long, default_value = "100")]
    loop_max: u32,
}

fn main() {
    let opts: Opts = Opts::parse();
    let pallet = Pallet::new(-2.0, -2.0, 4.0, 4.0, opts.pixel_pow, opts.pixel_pow);
    let loop_max = opts.loop_max;
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
