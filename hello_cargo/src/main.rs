use std::io; // This is a trait. Will discuss in Chapter 14 of the book.

fn main() {
    loop {
        println!("Please input a Fibonacci sequence.");

        let mut fib = String::new();
        io::stdin()
            .read_line(&mut fib)
            .expect("Failed to read line");

        let fib: u64 = match fib.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result = fibonacci_recursive(fib);
        println!("result is {}", result);
        break;
    }
}

fn fibonacci_recursive(x: u64) -> u64 {
    if x < 2 { x } else { fibonacci_recursive(x - 1) + fibonacci_recursive(x - 2)}
}
