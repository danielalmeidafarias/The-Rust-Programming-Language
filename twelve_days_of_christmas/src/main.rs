use std::io;

fn main() {
    println!("Welcome to the Rust Twelve Days of Christmas generator!");
    println!("Please, type the day you want(1-12)");

    let mut days: String = String::new();
    io::stdin().read_line(&mut days).expect("Error on reading the value");

    let mut days: Result<u32, _> = days.trim().parse();


    twelve_days_of_christmas(3);

}

fn twelve_days_of_christmas(days: u16) -> [String; 12] {

    let verses: [String; 12] = [
    "A partridge in a pear tree".to_string(), 
    "Two turtle doves".to_string(),
    "Three French hens".to_string(),
    "four calling birds".to_string(),
    "five gold rings".to_string(),
    "six geese a-laying".to_string(),
    "seven swans a-swimming".to_string(), 
    "eight maids a-milking".to_string(), 
    "nine ladies dancing".to_string(),
    "ten lords a-leaping".to_string(),
    "eleven pipers piping".to_string(),
    "twelve drummers drumming".to_string(),
    ];


}
