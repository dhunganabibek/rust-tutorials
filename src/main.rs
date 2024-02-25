pub mod util;

use util::can_drive;


fn main() {
    let age_to_drive  = 16 as u8;
    let mut my_input = String::from("");
    println!("Enter your age: ");
    std::io::stdin().read_line(&mut my_input).unwrap();
    println!("You entered: {}", my_input);


    if my_input.trim().parse::<u8>().unwrap() >= age_to_drive{
        println!("You can drive");
    }
    else {
        println!("You can't drive yet.")
    }
    let mut x = 0;

    loop {
        println!("Hello from rust");
        if x > 5 {
            break;
        }
        x += 1;
    }

    for loop in rust
    for num in 1..=10 {
        println!("Hello from rust {}", num);
    }
}

