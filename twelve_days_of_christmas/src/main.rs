fn main() {
    println!("Welcome to the Rust Twelve Days of Christmas generator!");

    twelve_days_of_christmas();

}

fn twelve_days_of_christmas() {

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

    for i in 0..12 {

        println!("On the {}nth day of Christmas my true lover sent to me", i + 1);

        for j in 0..=i {
            let x = i - j;
            println!("{}", verses[x]);
        }

        println!("");
    }


}