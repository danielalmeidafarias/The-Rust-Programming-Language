fn main() {

    // let guess = "43".parse().expect("Not a number"); // Error - Need an explicit type
    let guess: u32 = "43".parse().expect("Not a number");
    println!("{guess}");

    // 1 - Scalar types

        // 1.1 - Integer types
            // u = unsigned(always +); i = signed(may be + or -)
            // i/u8, i/u16, i/u32, i/u64, i/u128, i/usize 
        let integer_number: u32 = 43;
        println!("{integer_number}");

        // let overflowed_number: u8 = 1000; // Error - Overflow - out of the type range
        // println!("{overflowed_number}");

        // 1.2 - Floating-point types
            // f32, f64(default)
        let floating_number: f64 = 4.545;
        println!("{floating_number}");

        // Division
        // let division: f64 = 5 / 2.5; // Error - Cannot divide integer by float

        let quotient = 56.7 / 32.2; // Floating-point division
        println!("{quotient}"); // Results in 1.76086...

        let truncated = -5 / 3; // Integer division
        println!("{truncated}");// Results in -1,  the nearst integer

        // 1.3 - Boolean type
            // bool
        let verdadeiro: bool = true;
        println!("{verdadeiro}");

        // 1.4 - Character type
            // char
        let heart_eyed_cat: char = '😻'; // single quote
        println!("{heart_eyed_cat}"); // double quote

    // 2 - Compound Types

        // 2.1 - Tuple type
            // () unit = empty tuple
            // diferent type values
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, z) = tup; // destructuring
        let w: i32 = tup.0;
        print!("{x}, {y}, {z}, {w}");

        // 2.2 - Array type
            // unique type value
            // fixed length
        let array:[i32; 5] = [1, 2, 3, 4, 5];
        let array_element = array[1];
        print!("{array_element}");


}
