use std::fmt::Debug;


fn main() -> (){
  #[derive(Debug)]
  struct FullName {
    first_name: String,
    last_name: String
  };

  let full_name = FullName{
    first_name: String::from("Bibek"),
    last_name: String::from("Dhungana")
  };
  
  println!("{:?}", full_name);

  let point: (i8, i8) = (3, 5);
  let (x,y) = point;
  let greeting = "Hello, World!";
  let mut name = String::from("Bibek Dhungana");
  name = String::from("Bibek");
  let result = format!("{} {}", greeting, name);
  print!("{}", result);
  let multiply_result = multiply(10, 20);
  println!("{}", multiply_result);
}

fn multiply(a: i32, b: i32) -> i32{
  return a * b;
}