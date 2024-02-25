
trait Animal{

}
struct Dog{

}
impl Animal for Dog{}
struct Cat{}


struct Person<PetType: Animal>{
    first_name:String,
    pet:PetType
}

pub fn test_mod(){
    let p1 = Person{
        first_name:String::from("John"),
        pet: Dog{}
    };
}

