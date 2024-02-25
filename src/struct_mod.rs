#[derive(Debug)]
#[allow(dead_code)]
pub struct Person {
    first_name: String,
    last_name: String,
    birth_year: u16,
    birth_month: u8
}

impl Person {
    pub fn info(&self) -> String{
        format!("First Name: {0} \nLast Name: {1}\n Birth Year: {2}\nBirth_month:{3}", 
        self.first_name, 
        self.last_name, 
        self.birth_year,
        self.birth_month)
    
    }
}

pub fn test_struct(){
    let mut p1 = Person{
        first_name: "Bibek".to_string(), 
        last_name: "Dhungana".to_string(),
        birth_year: 1997,
        birth_month: 11
    };
    p1.first_name = "Test".to_string();

    println!("{}", p1.info());

    // println!("First Name: {0} \nLast Name: {1}\n Birth Year: {2}\nBirth_month:{3}", 
    // p1.first_name, 
    // p1.last_name, 
    // p1.birth_year,
    // p1.birth_month);

    // println!("{:?}", p1);
    
}