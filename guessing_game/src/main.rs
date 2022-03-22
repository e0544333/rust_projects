use rand::Rng;
use std::cmp::Ordering;
use std::io; // This is a trait. Will discuss in Chapter 14 of the book.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); // guess is binded mutable.
                                       // in cases where you’re using large data structures, mutating an instance in place may be faster than copying and returning newly allocated instances.

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // Example of shadow feature of Rust.
           // It allows us to reuse the guess variable name rather than forcing us to create two unique variables.

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            // Rust infers that secret_number is also a unsigned 32 integer.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
