use std::io;

fn celcuis_to_fahrenheit(unit: f32) -> f32 {
    (1.8 * unit) + 32.0
}

fn fahrenheit_to_celcius(unit: f32) -> f32 {
    (unit - 32.0) / 1.8
}
fn main() {
    let mut choice = String::new();
    println!("Enter conversion option");
    println!("1. Celcius to Fahrenheit");
    println!("2. Fahrenheit to Celcius");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter a number!!");
            return;
        }
    };
    let mut temperature_value = String::new();

    match choice {
        1 => {
            println!("Input temperature");
            io::stdin()
                .read_line(&mut temperature_value)
                .expect("Failed to read line");
            let temperature_value: f32 = match temperature_value.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input please enter a number");
                    return;
                }
            };
            let fahreinheit = celcuis_to_fahrenheit(temperature_value);
            println!(
                "{} celcius in fahrenheit is {}",
                temperature_value, fahreinheit
            );
        }
        2 => {}
        _ => println!("Invalid choice, please enter between choice 1 or 2"),
    }
    let cel: f32 = 42.0;
    let fah: f32 = 212.0;
    let fahrenheit = celcuis_to_fahrenheit(cel);
    let celcuis = fahrenheit_to_celcius(fah);
    println!("Temperature in fahrenheit is: {}", fahrenheit);
    println!("Temperature in celcus is: {}", celcuis);
}
