
enum CharacterType {
    Archer,
    Warrior,
    Mage
}

impl ToString for CharacterType {
    fn to_string(&self) -> String {
        match self {
            CharacterType::Archer => "Archer",
            CharacterType::Warrior => "Warrior",
            CharacterType::Mage => "Mage"
        }.to_string()
    } 
}


pub fn test_enum(){
    let mut opt1: Option<u8> = None;
    opt1 = Some(10);
    println!("opt1: {}", opt1.unwrap());
    println!("test_enum");

    let character = CharacterType::Archer;
    println!("character: {}", character.to_string());

}