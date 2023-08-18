use std::io;
use std::io::Write;
use std::num::ParseFloatError;

pub fn get_a() -> Result<f64, io::Error> {
    let mut a_str = String::new();
    let mut stdout = io::stdout();
    print!("Введите a: ");
    stdout.flush().expect("Faild to flush output");
    io::stdin().read_line(&mut a_str)?;

    let a: f64 = a_str
        .trim()
        .parse()
        .map_err(|e: ParseFloatError| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    Ok(a)
}
