use std::num::ParseIntError;

pub fn print_parsed_values() {
    let inputs = vec!["10", "15", "-1", "16", "42"];

    for input in inputs {
        match as_hex_string(input) {
            Ok(hex) => print!("{} ", hex),
            Err(_) => print!("{} ", input),
        }
    }
    println!();
}

fn as_hex_string(input: &str) -> Result<String, ParseIntError> {
    let value:u32 = input.parse()?;

    Ok(format!("{:02x}", value))
}