use std::io;

fn celsius_farenheit(deg: f32) -> f32 {
    deg*1.8 + 32.0
}

fn farenheit_celsius(deg: f32) -> f32 {
    (deg - 32.0) / 1.8
}

fn main() {
    loop {
        println!("Enter a value:");

        let mut degree = String::new();

        io::stdin().read_line(&mut degree).expect("failed to get value.");

        let degree: f32 = match degree.trim().parse() {
            Ok(val) => val,
            Err(_) => continue
        };

        println!("What conversion do you want? (F to C, C to F)");

        let mut conversion = String::new();

        io::stdin().read_line(&mut conversion).expect("Failed to get value.");

        let result: f32;

        match conversion.as_str().trim() {
            "F to C" => result = farenheit_celsius(degree),
            "C to F" => result = celsius_farenheit(degree),
            _ => {
                println!("Unrecognised conversion.");
                continue;
            }
        };

        println!("The result is: {result}");
    }
}
