fn main() {

    loop_function();
    
    while_function();

    for_function();

}

fn loop_function() {

    let mut count = 0;
    // labeled loop
    // especify the break and continue command
    'counting_up: loop {

        println!("count = {count}");
        let mut remaining = 10;
        loop {

            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

}

fn while_function() {

    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("{number}");
}

fn for_function() {

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }

    // Range - 1..4 = [1,2,3]
    // .rev() - [1,2,3] = [3,2,1]
    for number in (1..4).rev() {
        println!("{number}")
    }

}
