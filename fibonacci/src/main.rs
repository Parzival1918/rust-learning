use std::io;

fn main() {
    let mut n = String::new();

    // Get the user to input the n number of fibonacci numbers to print
    println!("Enter the number of fibonacci numbers to print: ");
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line.");

    // Convert the input to an integer
    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_)  => {
            println!("Please enter a valid number.");
            return;
        },
    };

    let fib_result = fib(n);

    println!("The value is {fib_result}")
}

fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n
    }
    fib(n-1) + fib(n-2)
}