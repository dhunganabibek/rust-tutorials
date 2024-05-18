
fn main(){
  let greeting = "Hello, World!";
  let mut name = String::from("Bibek Dhungana");
  name = String::from("Bibek");
  let result = format!("{} {}", greeting, name);
  print!("{}", result);
}