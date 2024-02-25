use std::collections::HashSet;

pub fn test_map_and_set() {

   let mut my_set = HashSet::<String>::new();
   my_set.insert("pi".to_string());
   my_set.insert("pi".to_string());
   my_set.insert("e".to_string());
   
   for value in my_set {
      println!("{}", value);
   }

   // let mut my_map = HashMap::<String, f32>::new(); 
   // my_map.insert("pi".to_string(), 3.14159);
   // my_map.insert("e".to_string(), 2.71828);
   //  my_map.insert("pi".to_string(), 3.14159);
   // my_map.insert("e".to_string(), 2.71828);
   // // println!("my_map: {:#?}", my_map);

   // for (key, value) in my_map {
   //    println!("{}: {}", key, value);
   // }
}