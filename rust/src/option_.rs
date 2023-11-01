pub fn test_option_type() -> Option<u8> {
    let mut opt1: Option<u8> = None;
    opt1 = Some(7);
    return opt1;
}

pub fn test_option_string() -> Option<String> {
    let mut opt1 = None;
    opt1 = Some("Shourya Sharma".to_string());
    return opt1;
}
// #[derive(Debug)]
pub enum CharacterType {
    Archer,
    Warrior,
    Mage,
}

impl ToString for CharacterType {
    fn to_string(&self) -> String {
        match self {
            CharacterType::Archer => "Archer".to_string(),
            CharacterType::Mage => "Mage".to_string(),
            CharacterType::Warrior => "Warrior".to_string(),
        }
    }
}

pub fn test_option_chartype() -> Option<CharacterType> {
    let mut chartype = None;
    chartype = Some(CharacterType::Mage);
    return chartype;
}

#[cfg(test)]
mod test {

    #[test]
    fn test_opt() {
        let result = super::test_option_type();
        println!("{}", result.unwrap());
        println!("{}", super::test_option_string().unwrap());
        println!("{:?}", super::test_option_chartype().unwrap().to_string());
        if super::test_option_chartype().is_some() {
            println!("User has selected something")
        } else {
            println!("Nothing is selected")
        }
    }
}
