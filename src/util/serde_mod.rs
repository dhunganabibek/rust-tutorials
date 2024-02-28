use serde::{Serialize, Deserialize};
use serde_json::to_string;

pub fn test_serde(){
    let dof = Dog {
        name: "Rusty".to_string(),
        year_born: 2018
    };
    let dog_json = to_string(&dof);

    if (dog_json.is_ok()){
        println!("Dog JSON: {}", dog_json.unwrap());
    } else {
        println!("Error: {}", dog_json.err().unwrap());
    }
} 

#[derive(Serialize, Deserialize)]
struct Dog {
    name: String,
    year_born: i32
}