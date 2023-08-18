use std::io::Result;

pub fn find_discriminant(a: f64, b: f64, c: f64) -> Result<f64> {
    let d = b * b - (4.0 * a * c);
    Ok(d)
}
