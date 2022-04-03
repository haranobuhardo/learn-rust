use std::{io, thread, time};

enum TempUnit {
    Celcius,
    Fahreheit
}

fn main() {
    let sleep_time = time::Duration::from_millis(200);
    println!("Welcome to Rust Temperature Converter V0.1");
    thread::sleep(sleep_time);
    println!("Created by Hardo - 2022");
    thread::sleep(sleep_time);

    print!("Starting application...");
    thread::sleep(sleep_time);
    print!(".");
    thread::sleep(sleep_time);
    print!(".");
    thread::sleep(sleep_time);
    print!(".");
    thread::sleep(sleep_time);
    println!(".");
    thread::sleep(sleep_time);

    let mut temp_unit = String::new();
    let mut temp_value = String::new();

    println!("Choose initial temperature (C or F): ");
    io::stdin()
        .read_line(&mut temp_unit)
        .expect("Failed to read the line!");

    let temp_unit:char = temp_unit
        .chars()
        .next()
        .expect("Temperature unit is not char!");
    
    if !(temp_unit == 'F' || temp_unit == 'C') {
        panic!("Only support Fahrenheit (F) and Celcius (C) for now!");
    }
    
    println!("Enter current temperature value: ");
    io::stdin()
        .read_line(&mut temp_value)
        .expect("Failed to read the line!");

    let temp_value:f32 = temp_value
        .trim()
        .parse()
        .expect("Index ordered was not a number!");

    println!("Function output: {}", temp_convert(&temp_value, &temp_unit));
    
}


fn temp_convert(temp: &f32, unit: &char) -> String {

    let converted_temp = format!("{}{}", temp, unit);
    println!("Initial temperature: {}", converted_temp);

    let temp_2:f32;
    let unit_2:char;

    match unit {
        'F' => {
            temp_2 = (temp - 32.0) * 5.0 / 9.0;
            unit_2 = 'C';
        },
        'C' => {
            temp_2 = (temp * 9.0 / 5.0) + 32.0;
            unit_2 = 'F';
        },
        _ => panic!("Undefined temperature unit!"),
    }

    let converted_temp = format!("{}{}", temp_2, unit_2);
    println!("Converted temperature: {}", converted_temp);

    return converted_temp
}