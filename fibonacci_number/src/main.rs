use std::io;

fn main() {
    println!("Welcome to Rust Fibonacci Generator!");

    loop {
        
        println!("Set the nth position of the fibonacci number you want(starts with 0)");

        let mut nth: String = String::new(); 

        io::stdin().read_line(&mut nth).expect("Error on reading the value");

        let nth: Result<u128, _> = nth.trim().parse();

        match nth {
            Ok(n) => {
                let f = fibonacci(n);
                println!("Your number is {}", f);
            }
            Err(e) => {
                println!("Error: {}", e)
            }
        }

        println!("Do you want to take another fibonacci number?");
        println!("y(yes) / another key(no)");
        let mut loop_handler: String = String::new();

        io::stdin().read_line(&mut loop_handler).expect("Error on reading the value");

        let loop_handler = loop_handler.trim();

        if loop_handler == "y" {
            continue;
        } else {
            println!("Bye!");
            break;
        }

    }



}

fn fibonacci(n: u128) -> u128 {

    let mut while_handler = n;
    let mut f: u128 = 0;

  
    if while_handler < 1 {
        f = 1;
    } else {

        let mut handler1: u128 = 1;
        let mut handler2: u128 = 1;


        while while_handler > 1 {

            f = handler1 + handler2;

            handler2 = handler1;

            handler1 = f;

            while_handler = while_handler - 1;


        }


    }

    return f;


}