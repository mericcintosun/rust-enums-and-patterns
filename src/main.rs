use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if b != 0.0 {
                a / b
            } else {
                println!("Error: Division by zero!");
                0.0
            }
        }
    }
}

fn main() {
    println!("Rust Hesap Makinesi");

    println!("İlk sayıyı girin:");
    let mut first_input = String::new();
    io::stdin()
        .read_line(&mut first_input)
        .expect("Okuma hatası");
    let first_number: f64 = match first_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Geçersiz sayı girdiniz.");
            return;
        }
    };

    println!("Yapmak istediğiniz işlemi girin (+, -, *, /):");
    let mut operation_input = String::new();
    io::stdin()
        .read_line(&mut operation_input)
        .expect("Okuma hatası");
    let operation = operation_input.trim();

    println!("İkinci sayıyı girin:");
    let mut second_input = String::new();
    io::stdin()
        .read_line(&mut second_input)
        .expect("Okuma hatası");
    let second_number: f64 = match second_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Geçersiz sayı girdiniz.");
            return;
        }
    };

    let op = match operation {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Geçersiz işlem girdiniz.");
            return;
        }
    };

    let result = calculate(op);

    println!("Sonuç: {}", result);
}
