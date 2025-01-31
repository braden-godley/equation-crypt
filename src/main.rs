use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);

    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    let my_equation = |x| (x as f32).cos() * 10.0;

    let encrypted = encrypt(&content, my_equation)?;

    println!("Encrypted: {}", encrypted);

    let decrypted = decrypt(&encrypted, my_equation)?;

    println!("Decrypted: {}", decrypted);

    Ok(())
}

type Equation = fn(i32) -> f32;

fn algorithm(input: &str, equation: Equation, mult: i32) -> std::io::Result<String> {
    let mut out = String::new();

    let mut j = 0;
    for c in input.chars().enumerate() {
        let byte = c.1 as u8;
        let offset: i32 = equation(j).floor() as i32 * mult;
        let total = byte as i32 + offset as i32;
        let result = (total % 256) as u8;
        println!("{} + {} -> {} -> {}", byte, offset, total, result);
        out.push(result as char);
        j += 1;
    }

    Ok(out)
}

fn encrypt(input: &str, equation: Equation) -> std::io::Result<String> {
    algorithm(input, equation, 1)
}

fn decrypt(input: &str, equation: Equation) -> std::io::Result<String> {
    algorithm(input, equation, -1)
}

