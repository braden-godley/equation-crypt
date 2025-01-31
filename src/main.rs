use std::fs::File;
use std::io::{self, BufReader, Read};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    println!("Input text:\n{}", input);

    let my_equation = |x| x as f32 + (x as f32).cos();

    let encrypted = encrypt(&input, my_equation)?;
    println!("Encrypted:\n{}", encrypted);

    let decrypted = decrypt(&encrypted, my_equation)?;
    println!("Decrypted:\n{}", decrypted);

    Ok(())
}

type Equation = fn(i32) -> f32;

fn algorithm(input: &str, equation: Equation, mult: i32) -> io::Result<String> {
    let mut output = String::new();

    for (j, c) in input.chars().enumerate() {
        let byte = c as u8;
        let offset: i32 = equation(j.try_into().unwrap()).floor() as i32 * mult;
        let total = byte as i32 + offset;
        let result = (total % 256) as u8;

        //println!("{} + {} -> {} -> {}", byte, offset, total, result);
        output.push(result as char);
    }

    Ok(output)
}

fn encrypt(input: &str, equation: Equation) -> io::Result<String> {
    algorithm(input, equation, 1)
}

fn decrypt(input: &str, equation: Equation) -> io::Result<String> {
    algorithm(input, equation, -1)
}
