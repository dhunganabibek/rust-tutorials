use std::vec;

pub fn test_vector(){
    let mut  v: Vec<i32> = Vec::new();

    for i in 0..10{
        v.push(i);
    }

    println!("{:?}",v.len());
    println!("{:?}",v.capacity());
    println!("{:?}",&(&v).as_slice()[0..5]);
    println!("{:?}",v.get(1).unwrap());

    let names = vec!["Bibek", "Dhungana", "Hello"];
    for name in names.clone(){
        println!("{}",name);
    }
    println!("{:?}",names);

    let mut cars: Vec<Car> = Vec::new();
    cars.push(Car{
        manufacturer: String::from("Toyota"),
        model: String::from("Corolla"),
    });
    cars.push(Car{
        manufacturer: String::from("Honda"),
        model: String::from("Civic"),
    });
    println!("{:?}",cars);
}

#[derive(Debug)]
struct Car{
    manufacturer: String,
    model: String,
}