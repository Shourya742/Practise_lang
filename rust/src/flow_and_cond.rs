#[cfg(test)]
mod test{
    #[test]
    fn flow_and_cond(){
        let age_to_drive=16u8;
        println!("Enter the person's age!");
        let myinput = &mut String::from("");
        std::io::stdin().read_line(myinput).unwrap();
        let age = myinput.replace("\n", "").parse::<u8>().unwrap();
        if age > age_to_drive {
            println!("Drive");
        } else {
            println!("Dont Drive");
        }

        let driver_license = if age >= 16 {true} else {false};
        let mut val=0;
        while val<=age {
            println!("{}",val);
            val+=1;
        }
        let ages=[12,45,12,42,64,46];
        
        for value in ages {
            if age > value {
                println!("Dont drive");
            } else {
                println!("Drive");
            }
        }
    }
}