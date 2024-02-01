use dialoguer::{Select, Input};
use dialoguer::theme::ColorfulTheme;
use console::Style;

fn celsius_farenheit(deg: f32) -> f32 {
    deg*1.8 + 32.0
}

fn farenheit_celsius(deg: f32) -> f32 {
    (deg - 32.0) / 1.8
}

fn main() {
    let output_style = Style::new().cyan().bold();

    loop {
        let degree: String = Input::with_theme(&ColorfulTheme::default()).with_prompt("Enter a value")
            .validate_with(|input: &String| -> Result<(), &str> {
                match input.trim().parse::<f32>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err("Not valid value") 
                }
            }).interact_text().unwrap();

        let degree: f32 = match degree.trim().parse() {
            Ok(val) => val,
            Err(_) => continue
        };

        let select_opts = vec!["Celsius to Farenheit", "Farenheit to Celsius"];

        let selection = Select::with_theme(&ColorfulTheme::default()).with_prompt("Select the conversion type").default(0)
            .items(&select_opts).interact().unwrap();

        let result: f32;
        match selection {
            0 => result = celsius_farenheit(degree),
            1 => result = farenheit_celsius(degree),
            _ => result = 0.0
        }

        println!("The conversion to other degree scale is: {}", output_style.apply_to(result));
    }
}
