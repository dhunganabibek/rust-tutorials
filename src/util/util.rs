
pub fn get_full_name (first: &str, last: &str) -> String{
    let full_name = format!("{0} {1}", first, last);
    return full_name;
}


pub fn add(a: i32, b:i32) -> i32{
    return  a + b;
}

pub fn can_drive(age: i32) -> bool{
    age >= 16
}