use dialoguer::{theme::ColorfulTheme, Select};
const PROMPT_ITEMS: [&str; 2] = ["°C to °F", "°F to °C"];
const RESULT_ACCURACY: i32 = 2;
fn main() {
    let user_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select convert mode")
        .items(&PROMPT_ITEMS)
        .default(0)
        .interact()
        .unwrap();
    let input_temperature = dialoguer::Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter temperature")
        .validate_with(|input: &String| -> Result<(), &str> {
            match input.trim().parse::<f32>() {
                Ok(_) => Ok(()),
                Err(_) => Err("Please enter number only!"),
            }
        })
        .interact_text()
        .unwrap();
    let input_temperature: f32 = input_temperature.trim().parse().unwrap();
    let result = {
        if user_selection == 0 {
            celsius_to_fahrenheit(input_temperature)
        } else {
            fahrenheit_to_celsius(input_temperature)
        }
    };
    println!("{result}{}", if user_selection == 0 { "°F" } else { "°C" });
}
fn celsius_to_fahrenheit(input: f32) -> f32 {
    let result = (input * 1.8) + 32.0;
    round_result(result, RESULT_ACCURACY)
}
fn fahrenheit_to_celsius(input: f32) -> f32 {
    let result = (input - 32.0) / 1.8;
    round_result(result, RESULT_ACCURACY)
}
fn round_result(input: f32, accuracy: i32) -> f32 {
    let multiply = 10_f32.powi(accuracy);
    (input * multiply).round() / multiply
}
