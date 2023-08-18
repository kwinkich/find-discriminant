use std::io;
use std::io::Write;
use std::num::ParseFloatError;

pub fn get_c() -> Result<f64, io::Error> {
    let mut c_str = String::new();
    let mut stdout = io::stdout();
    print!("Введите c: ");
    stdout.flush().expect("Faild to flush output");
    io::stdin().read_line(&mut c_str)?;

    let c: f64 = c_str
        .trim()
        .parse()
        .map_err(|e: ParseFloatError| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    Ok(c)
}
