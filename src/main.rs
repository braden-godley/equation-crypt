use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);

    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    let my_equation = |x| x * x;

    let encrypted = encrypt(&content, my_equation)?;

    println!("Encrypted: {}", encrypted);

    let decrypted = decrypt(&encrypted, my_equation)?;

    println!("Decrypted: {}", decrypted);

    Ok(())
}

type Equation = fn(i32) -> i32;

fn algorithm(input: &str, equation: Equation, mult: i32) -> std::io::Result<String> {
    let mut out = String::new();

    let chars = input.chars();

    let mut j = 0;
    for c in chars.enumerate() {
        let byte = c.1 as u8;
        let offset = equation(j) * mult;
        let total = byte as i32 + offset;
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

