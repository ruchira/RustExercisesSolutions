use std::io;

fn main() {
    loop {
        println!("Please input the temperature in degrees.");

        let mut temperature = String::new();

        io::stdin().read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let is_fahrenheit = is_temperature_scale_fahrenheit();

        if is_fahrenheit {
            let conversion = (temperature - 32.0) * 5.0 / 9.0;
            println!("{}°F = {}°C", temperature, conversion);
        } else {
            let conversion = temperature * 9.0 / 5.0 + 32.0;
            println!("{}°C = {}°F", temperature, conversion);
        }
    }
}

fn is_temperature_scale_fahrenheit() -> bool {
    loop {
        println!("Enter F for Fahrenheit or C for Celsius.");

        let mut scale = String::new();

        io::stdin().read_line(&mut scale)
            .expect("Failed to read line");

        let scale = scale.trim();

        if scale == "F" {
            return true
        } else if scale == "C" {
            return false
        } else {
            println!("Please enter 'F' or 'C'.");
            continue
        };
    }
}
