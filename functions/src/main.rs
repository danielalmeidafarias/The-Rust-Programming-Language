fn main() {

    println!("Hello, world!");
    another_function(5, -100);

    // Rust is an expression-based-language

    // Statements = instructions that perform some action and do not return a value
    // Expressions = evaluate to a resultant value

    let y = {
        let x = 3;
        x + 1
    }; 

    println!("The value of y is: {y}");
    
    let x = plus_one(5); 

    println!("The value of x is: {x}");

}

// You can call a function anywhere on the code, since its in the expected scope
fn another_function(x: u32, y: i64) {
    // snake_case names 
    println!("Another function!");
    println!("The value of x is {x} and the value of y is {y}");

}

// Functions with return values
fn plus_one(x: i32) -> i32 {

    x + 1 // without semicolons

}