# Equation-Crypt

This is the implementation of a random idea for encrypting strings.

## The Algorithm

Basically, you can create any custom mathematical function that takes an integer and produces a float.

The algorithm acts like a dynamic caesar cipher, which operates on 8-bit char values.

When encrypting, it determines the cipher offset for each character by plugging the character's position in the string into the equation.

For instance, suppose the function is: `f(x) = x + cos(x)` and the input string is "Apple"

The input string's 0-th character is A, which is 65 in its 8-bit ASCII representation. We plug the index, 0, into the equation:

`f(0) = 0.0 + 1.0 = 1.0`

We floor the output of the function to get a whole number offset: `floor(f(0)) = 1`

Then, add the offset to the 8-bit value of the 0-th character: `65 + 1 = 66`. 66 in ASCII is 'B', so that's the first character of the ciphertext.

This process is repeated for each character in the string. You take its ASCII value, and then add the floored function at the i-th position. If it goes below zero or above 255, simply wrap the value like you would a caesar cipher.

The decryption algorithm is the same, except it performs the offset in the opposite direction.

## How to run

1. Download this repository
2. Use `cargo run` in the repository.

It should show you the input text, the ciphertext, and the decrypted ciphertext (which should be the same as the input text).

Change `input.txt` to change the input text. 

Modify `my_equation` in `src/main.rs` to use a different equation.

## Example usage

```bash
bgodley@machine:~/git/equation-encrypt$ cargo run
   Compiling equation-crypt v0.1.0 (/home/bgodley/git/equation-encrypt)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/equation-crypt`
Input text:
Apple
line1
line2
line3
test


Encrypted:
Bqqnhrpum:xv|sA~|┬éyH ┬î~┬ì┬Ä%&
Decrypted:
Apple
line1
line2
line3
test

```
