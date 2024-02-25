fn main() {
    let a = 5;
    let pi = 3.9415;

    let ans = a + pi as i32;

    let my_str  = 'A';
    let my_str2 = "Hello World!";

    let my_tup = ("Hello", 5, 3.1415);
    let ages:[u128; 5] = [1,2,3,4,5];

    //slices
    let slice = &ages[1..3];


    
    println!("{:?}", slice);
}
