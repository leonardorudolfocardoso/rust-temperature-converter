use std::io;

fn main() {
    println!("Temperature converter");

    'temperature_to_convert: loop {
        let mut temperature_to_convert = String::new();
        println!("Input temperature to convert");
        io::stdin()
            .read_line(&mut temperature_to_convert)
            .expect("Not able to read temperature. ");

        let temperature_to_convert: f32 = match temperature_to_convert.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not able to convert \"{}\". Try again. ", temperature_to_convert.trim());
                continue 'temperature_to_convert
            },
        };
    

            'temperature_target: loop {
                let mut temperature_target = String::new();
        
                println!("Press C to convert to celsius or F to convert to fahrenheit");
                io::stdin()
                    .read_line(&mut temperature_target)
                    .expect("Not able to read temperature. ");
                
                match temperature_target {
                    _ if temperature_target.trim().to_lowercase() == "c" => {
                        println!("{temperature_to_convert}°F is {}°C", fahrenheit_to_celsius(temperature_to_convert));
                    },
                    _ if temperature_target.trim().to_lowercase() == "f" => {
                        println!("{temperature_to_convert}°C is {}°F", celsius_to_fahrenheit(temperature_to_convert));
                        break 'temperature_target;
                    },
                    _ => print!("Invalid temperature {temperature_target}")
                }
            }
    };
}

fn celsius_to_fahrenheit (temperature: f32) -> f32 {
    temperature * 9./5. + 32.
}

fn fahrenheit_to_celsius (temperature: f32) -> f32 {
    (temperature - 32.) * 5./9.
}
