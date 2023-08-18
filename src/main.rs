use std::io;
use std::io::Result;
mod find_discriminant;
mod find_x;
mod get_nums;

fn read_nums() -> Result<(f64, f64, f64)> {
    let a = get_nums::get_a()?;
    let b = get_nums::get_b()?;
    let c = get_nums::get_c()?;
    Ok((a, b, c))
}

fn main() {
    loop {
        match read_nums() {
            Ok((a, b, c)) => match find_discriminant::find_discriminant(a, b, c) {
                Ok(d) => {
                    println!("Дискриминант: {}", d);

                    if let Some((x1, x2)) = find_x::find_x(a, b, d) {
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
