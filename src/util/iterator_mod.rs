pub fn test_iterator(){
    let fruits = vec!["banana", "apple", "pear"];

    let fruits_iter = fruits.iter();
    
    for fruit in fruits_iter {
        println!("{}", fruit);
    }
}