pub mod util;

use util::{get_full_name, add};


fn main() {
    let full_name = get_full_name("John", "Doe");
    println!("Full Name: {}", full_name);

    let sum = add(10, 20);
    println!("Sum: {}", sum);
}

