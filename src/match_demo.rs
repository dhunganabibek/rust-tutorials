pub fn test_match(){
    let my_age = 35;

    match my_age {
        35 => {
            println!("You are 35 years old");
        },
        _ => {
            println!("You are not 35 years old");
        }
    }
}