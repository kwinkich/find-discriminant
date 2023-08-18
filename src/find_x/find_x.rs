pub fn find_x(a: f64, b: f64, d: f64) -> Option<(f64, f64)> {
    if d > 0.0 {
        let d_sqrt = d.sqrt();
        let x1 = (-b + d_sqrt) / (2.0 * a);
        let x2 = (-b - d_sqrt) / (2.0 * a);
        Some((x1, x2))
    } else if d == 0.0 {
        let x = -b / (2.0 * a);
        Some((x, x))
    } else {
        None
    }
}
