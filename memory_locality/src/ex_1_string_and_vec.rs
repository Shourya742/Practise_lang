


#[cfg(test)]
mod string_and_vec_tests {
    use r3bl_tui::{fg_light_yellow_green, fg_lizard_green};

    #[test]
    fn mem_layout_string() {
        fg_lizard_green("\n=== String Memory Layout Example ===").println();

        let s = String::from("0123456789");

        fg_light_yellow_green("\nSafety accessing String metadata:").println();

        println!(" ptr: {:p}", s.as_ptr());
        println!(" len: {}", s.len());
        println!(" cap: {}", s.capacity());

        fg_light_yellow_green("\nUnsafety accessing String as Vec<u8> (hex dump):").println();

        println!("{:?}", unsafe {
            std::mem::transmute::<String, Vec<u8>>(s);
        });

        {
            fg_light_yellow_green("\nAccessing String with into_raw_parts():").println();
            let s = String::from("0123456789");
            let (ptr, len, cap) = s.into_raw_parts();
            println!(" ptr: {:p}", ptr);
            println!(" len: {}", len);
            println!(" cap: {}", cap);
        }

    }


    #[test]
    fn mem_layout_str_slice() {
        fg_lizard_green("\n=== &str Memory Layout Example 1 ===").println();

        let s = "Hello, World";

        unsafe {
            let raw_part: (*const u8, usize) = std::mem::transmute(s);

            fg_light_yellow_green("\n&str memory layout").println();
            println!(" ptr: {:p}", raw_part.0);
            println!(" len: {}", raw_part.1);

            fg_light_yellow_green("\nSafely accessing &str metadata:").println();
            println!(" ptr: {:p}", s.as_ptr());
            println!(" len: {}", s.len());
        }
    }

    #[test]
    fn mem_layout_str_slice_2() {
        fg_lizard_green("\n=== &str Memory Layout Example 2 ===").println();

        let owned = String::from("Hello, world!");
        let slice = &owned[0..5];

        let slice_ptr = slice.as_ptr();
        let slice_len = slice.len();

        let owned_ptr = owned.as_ptr();
        let owned_len = owned.len();
        let owned_capacity = owned.capacity();

        fg_light_yellow_green(
            "\nComparing owned String and &str slice (safely):"
        ).println();
        println!("  String ptr: {:p}", owned_ptr);
        println!("  &str ptr:   {:p}", slice_ptr);
        println!(
            "  String points to same memory as slice: {}",
            slice_ptr == owned_ptr
        );
        println!("  String len: {}, slice len: {}",
            owned_len, slice_len);
        println!("  String cap: {}", owned_capacity);

    }
}