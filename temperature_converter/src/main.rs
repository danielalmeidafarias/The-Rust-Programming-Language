use std::io;

fn main() {

    println!("Welcome to the Rust Temperature Converter!");

    loop {

        println!("Set your temperature standart");
        println!("Celcius(c) / Fahrenheit(f)");
        let mut standart: String = String::new();
        io::stdin().read_line(&mut standart).expect("Error on reading the standarts");

        let standart = standart.trim();

        let mut handler: String = String::new();
        
        if standart == "c".to_string() {
            println!("Please, type your temperature");
            let mut temperature: String = String::new();
            io::stdin().read_line(&mut temperature).expect("Error on reading the temperature");
        
            let temperature: &str = temperature.trim();
        
            let temperature: Result<f64, _> = temperature.parse();
        
            match temperature {
                Ok(n) => {
                    let converted_temperature = convert_to_fahrenheit(n);
                    println!("Your temperature is {} fahrenheit degress", converted_temperature);
                }
                Err(e) => {
                    println!("Error on converting {}", e);
                }
            }

            println!("Do you want to make another conversion?");
            println!("y(yes) / another_key(no)");
            io::stdin().read_line(&mut handler).expect("Error on reading the value");

            let handler = handler.trim();

            if handler == "y" {
                continue;
            } else {
                println!("Bye!");
                break;
            } 

    
        } else if standart == "f" {
            println!("Please, type your temperature");
            let mut temperature: String = String::new();
            io::stdin().read_line(&mut temperature).expect("Error on reading the temperature");
    
            let temperature: &str = temperature.trim();
            let temperature: Result<f64, _> = temperature.parse();
    
            match temperature {
                Ok(n) => {
                    let converted_temperature = convert_to_celcius(n);
                    let converted_temperature = format!("{:.1}", converted_temperature);
                    println!("Your temperature is {} celcius degress", converted_temperature);
                }
                Err(e) => {
                    println!("Error on converting {}", e);
                }
            }

            println!("Do you want to make another conversion?");
            println!("y(yes) / another_key(no)");

            io::stdin().read_line(&mut handler).expect("Error on reading the value");

            let handler = handler.trim();

            if handler == "y" {
                continue;
            } else {
                println!("Bye!");
                break;
            } 
            
        } else {
            println!("Please type a valid keyboad(c / f)");
            continue;
        }

    }

    
    


    
;
}

fn convert_to_fahrenheit(temperature: f64) -> f64{

    let fahrenheit: f64 = (temperature * 1.8) + 32.0;

    return fahrenheit;

}

fn convert_to_celcius(temperature: f64) -> f64 {
    
    let celcius: f64 = (temperature - 32.0) / 1.8;

    return celcius;

}
