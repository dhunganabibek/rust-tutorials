pub fn test_vector(){
    let mut  v: Vec<i32> = Vec::new();

    for i in 0..10{
        v.push(i);
    }

    println!("{:?}",v.len());
    println!("{:?}",v.capacity());
    println!("{:?}",&(&v).as_slice()[0..5]);
}