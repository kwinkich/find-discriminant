use std::io;
use std::io::Write;
use std::num::ParseFloatError;

pub fn get_b() -> Result<f64, io::Error> {
    let mut b_str = String::new();
    let mut stdout = io::stdout();
    print!("Введите b: ");
    stdout.flush().expect("Faild to flush output");
    io::stdin().read_line(&mut b_str)?;

    let b: f64 = b_str
        .trim()
        .parse()
        .map_err(|e: ParseFloatError| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    Ok(b)
}
