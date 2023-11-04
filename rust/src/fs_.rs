#[cfg(test)]
mod test {

    use std::fs;
    #[test]
    fn test_create_dir() {
        let path = "./data";
        let my_path = std::path::Path::new(path);
        if my_path.exists() {
            println!("Directory already exists! Skipping creation ..");
            return;
        }
        let create_dir_result = fs::create_dir(path);
        if create_dir_result.is_ok() {
            println!("Created new data directory");
        } else {
            println!(
                "Some problem occured creating data directory,{:?}",
                create_dir_result.err()
            );
        }
    }

    #[test]
    fn test_create_file() {
        let path = "./data/file01.txt";
        let text = b"Hello World!!";
        _ = fs::write(path, text);

        // _ = fs::remove_file(path);
    }

    #[test]
    fn remove_dir() {
        let path = "./data";
        _ = std::fs::remove_dir_all(path);
    }

    #[test]
    fn read_somefile() {
        let file_to_read = "./data/file01.txt";
        let read_result = std::fs::read(file_to_read);
        let convert_bytes_to_string = |mut a: String, v: &u8| {
            let new_char = char::from(*v);
            a.push(new_char);
            return a;
        };
        if read_result.is_ok() {
            println!(
                "Data found is {}",
                read_result
                    .ok()
                    .unwrap()
                    .iter()
                    .fold(String::from(""), convert_bytes_to_string)
            );
        }
    }
}
