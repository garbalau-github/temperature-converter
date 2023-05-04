use std::io;
use std::process;

fn main() {
    println!("Welcome to Temperature Converter!");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");

    let mut option = String::new();

    io::stdin().read_line(&mut option).expect("Invalid option");

    // Assign `option` after trimming whitespace
    let option = option.trim();

    match option {
        "1" => println!("1 is selected"),
        "2" => println!("2 is selected"),
        other => {
            println!("{} option is not supported", other);
            process::exit(0);
        }
    }

    let mut temperature = String::new();

    if option == "1" {
        println!("Enter Celsius:");
    }

    if option == "2" {
        println!("Enter Fahrenheit:");
    }

    io::stdin().read_line(&mut temperature).expect("Failed to read line");

    let temperature = convert_to_num(&temperature);

    if temperature == -1 {
        println!("Invalid number, try again!");
        process::exit(0);
    }

    println!("You entered {}", temperature);

    if option == "1" {
        println!("In Fahrenheit: {}", celcius_to_fahrenheit(temperature));
    }

    if option == "2" {
        println!("In Celsius: {}", fahrenheit_to_celsius(temperature));
    }
}

fn celcius_to_fahrenheit(celsius: i32) -> i32 {
    let fahrenheit = (celsius * 9 / 5) + 32;
    fahrenheit
}

fn fahrenheit_to_celsius(fahrenheit: i32) -> i32 {
    let celsius = (fahrenheit - 32) * 5 / 9;
    celsius
}

fn convert_to_num(value: &str) -> i32 {
    match value.trim().parse() {
       Ok(num) => num,
       Err(_) => {
           println!("Enter a number, please!");
           -1
       }
   }
}
