
struct Person {
    first: String,
    last: String,
}


pub fn test_closures(){
    let add = |x, y| x + y; 
    let result = add(20, 30);
    println!("The result is: {}", result);
    

    let mut  person = Person{
        first: "Bibek".to_string(),
        last: "Dhungana".to_string()
    };

    let mut modify_last_name = || {
        person.last = "Shrestha".to_string();
    };

    modify_last_name();
    person.first = "Bibek".to_string();

    println!("lastname: {}", person.last);
}