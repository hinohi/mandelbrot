pub fn cast_color(r: f64) -> u8 {
    if r < 0.0 {
        0
    } else if r > 1.0 {
        255
    } else {
        (255.0 * r).ceil() as u8
    }
}
