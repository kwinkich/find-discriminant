use std::io;
use std::io::Result;
use std::num::ParseFloatError;

fn get_nums() -> Result<(f64, f64, f64)> {
    let mut a_str = String::new();
    let mut b_str = String::new();
    let mut c_str = String::new();

    println!("Введите a: ");
    io::stdin().read_line(&mut a_str)?;

    println!("Введите b: ");
    io::stdin().read_line(&mut b_str)?;

    println!("Введите c: ");
    io::stdin().read_line(&mut c_str)?;

    let a: f64 = a_str
        .trim()
        .parse()
        .map_err(|e: ParseFloatError| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let b: f64 = b_str
        .trim()
        .parse()
        .map_err(|e: ParseFloatError| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let c: f64 = c_str
        .trim()
        .parse()
        .map_err(|e: ParseFloatError| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    Ok((a, b, c))
}

fn find_discriminant(a: f64, b: f64, c: f64) -> Result<f64> {
    let d = b * b - (4.0 * a * c);
    Ok(d)
}

fn find_x(a: f64, b: f64, d: f64) -> Option<(f64, f64)> {
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

fn main() {
    loop {
        match get_nums() {
            Ok((a, b, c)) => match find_discriminant(a, b, c) {
                Ok(d) => {
                    println!("Дискриминант: {}", d);

                    if let Some((x1, x2)) = find_x(a, b, d) {
                        println!("Корни: x1 = {}, x2 = {}\n", x1, x2);
                    } else {
                        println!("Уравнение не имеет действительных корней.\n");
                    }
                }
                Err(err) => {
                    eprintln!("Ошибка при вычислении дискриминанта: {}\n", err);
                }
            },
            Err(err) => {
                eprintln!("Ошибка: {}\n", err);
            }
        }

        println!("Введите 'exit' для завершения или нажмите Enter для продолжения.");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Не удалось прочитать строку");
        if input.trim() == "exit" {
            break;
        }
    }
}
