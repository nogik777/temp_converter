use std::io;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_kelvin(f: f64) -> f64 {
    (f + 459.67) * 5.0 / 9.0
}

fn kelvin_to_fahrenheit(k: f64) -> f64 {
    k * 9.0 / 5.0 - 459.67
}

fn kelvin_to_celsius(k: f64) -> f64 {
    k - 273.15
}

fn celsius_to_kelvin(c: f64) -> f64 {
    c + 273.15
}

fn one_more() -> bool {
     println!("One more conversion? (1 = yes 2 = no)");
                let mut yes_no = String::new();
                io::stdin().read_line(&mut yes_no).expect("Failed to read line");
                let yes_no: u32 = match yes_no.trim().parse() {
                    Ok(num) => num,
                    Err(_) => return false,
                };
                match yes_no {
                    1 => true,
                    2 => false,
                    _ => true,
                }
}

fn main() {
    println!("Temperature Conversion");

    loop {
        println!("Choose the conversion:");
        println!("1: Fahrenheit to Celsius");
        println!("2: Celsius to Fahrenheit");
        println!("3: Celsius to Kelvin");
        println!("4: Kelvin to Celsius");
        println!("5: Fahrenheit to Kelvin");
        println!("6: Kelvin to Fahrenheit");
        println!("7: Exit");

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
                println!();
                if one_more() {
                    continue
                } else { break }
            }
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
                println!();
                if one_more() {
                    continue
                } else { break }
            }
            3 => {
                println!("Enter temperature in Celsius:");
                let mut temp_c = String::new();
                io::stdin().read_line(&mut temp_c).expect("Failed to read line");
                let temp_c: f64 = match temp_c.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let temp_k = celsius_to_kelvin(temp_c);
                println!("{} °C is equal to {:.2} K", temp_c, temp_k);
                println!();
                if one_more() {
                    continue
                } else { break }
            }
            4 => {
                println!("Enter temperature in Kelvins:");
                let mut temp_k = String::new();
                io::stdin().read_line(&mut temp_k).expect("Failed to read line");
                let temp_k: f64 = match temp_k.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let temp_c = kelvin_to_celsius(temp_k);
                println!("{} K is equal to {:.2} °C", temp_k, temp_c);
                println!();
                if one_more() {
                    continue
                } else { break }
            }
            5 => {
                println!("Enter temperature in Fahrenheit:");
                let mut temp_f = String::new();
                io::stdin().read_line(&mut temp_f).expect("Failed to read line");
                let temp_f: f64 = match temp_f.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let temp_k = fahrenheit_to_kelvin(temp_f);
                println!("{} °F is equal to {:.2} K", temp_f, temp_k);
                println!();
                if one_more() {
                    continue
                } else { break }
            }
            6 => {
                println!("Enter temperature in Kelvins:");
                let mut temp_k = String::new();
                io::stdin().read_line(&mut temp_k).expect("Failed to read line");
                let temp_k: f64 = match temp_k.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let temp_f = kelvin_to_fahrenheit(temp_k);
                println!("{} K is equal to {:.2} °F", temp_k, temp_f);
                println!();
                if one_more() {
                    continue
                } else { break }
            }
            7 => break,
            _ => continue
        }
    }
}