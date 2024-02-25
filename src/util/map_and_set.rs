use std::collections::HashMap;

pub fn test_map_and_set() {
   let mut my_map = HashMap::<String, f32>::new(); 
   my_map.insert("pi".to_string(), 3.14159);
   my_map.insert("e".to_string(), 2.71828);
    my_map.insert("pi".to_string(), 3.14159);
   my_map.insert("e".to_string(), 2.71828);
   println!("my_map: {:#?}", my_map);
}