use std::io;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn main() {
    println!("Temperature Conversion");

    loop {
        println!("Choose the conversion:");
        println!("1: Fahrenheit to Celsius");
        println!("2: Celsius to Fahrenheit");
        println!("3: Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Enter temperature in Fahrenheit:");
                let mut temp_f = String::new();
                io::stdin().read_line(&mut temp_f).expect("Failed to read line");
                let temp_f: f64 = match temp_f.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let temp_c = fahrenheit_to_celsius(temp_f);
                println!("{} °F is equal to {:.2} °C", temp_f, temp_c);
            },
            2 => {
                println!("Enter temperature in Celsius:");
                let mut temp_c = String::new();
                io::stdin().read_line(&mut temp_c).expect("Failed to read line");
                let temp_c: f64 = match temp_c.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let temp_f = celsius_to_fahrenheit(temp_c);
                println!("{} °C is equal to {:.2} °F", temp_c, temp_f);
            },
            3 => break,
            _ => continue,
        }
    }
}