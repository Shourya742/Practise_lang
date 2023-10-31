#[cfg(test)]
mod test {
    #[test]
    fn d_types() {
        // let x: i8 = 255;
        let x: f32 = 255.0;
        let y: u8 = x as u8 - 5;
        println!("{:?}",y);

        let mut iamold: bool = true;
        iamold=false;
        println!("{}",iamold);

        // char are usually 4 bytes
        let mystr = 'A';
        println!("{}",mystr);
       
       let mut first_name:&str = "Shourya";
       first_name="Sharma";
       println!("{}",first_name);

       let name = ("Shourya","Sharma",7 as u8);
       println!("{:?}",name);

       let ages = [40,45,50,55,60,65,70];
       
       println!("{:?}",ages);

       let new_ages = &ages[1..4];
       println!("{:?}",new_ages);

    }
}