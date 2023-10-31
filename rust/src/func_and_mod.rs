
fn get_full_name(first:&str,last:&str)->String {
    let full_name = format!("{0} {1}",first,last);
    full_name
}

pub mod privatefns {
    pub fn get_age_plus_5(age:u16)->u16{
        age+5
    }
}
#[cfg(test)]
mod test {
    use crate::func_and_mod::privatefns;

    use super::get_full_name;

    #[test]
    fn func_and_mod(){
        let value = get_full_name("Hello", "Everyone");
        println!("{}",value);
        let new_age = privatefns::get_age_plus_5(2);
        println!("{}",new_age);
    }
}