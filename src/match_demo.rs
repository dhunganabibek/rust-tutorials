pub fn test_match(){
    let my_age: u8 = 36;


    match my_age {
        35 => println!("You are 35 years old"),
        _ => println!("You are not 35 years old")
        
    }
}