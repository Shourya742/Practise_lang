
mod test {

    #![allow(warnings)]

    use r3bl_tui::fg_lizard_green;
    use r3bl_tui::fg_light_yellow_green;


    struct Demo {
        a: u8,
        b: u32,
        c: u16
    }
    
    #[test]
    fn test_alignment() {
        let size = size_of::<Demo>();
        let align = align_of::<Demo>();
    
        assert_eq!(size, 8);
        assert_eq!(align, 4);

        fg_lizard_green(format!("\nSize of Demo: {size}")).println();
        fg_light_yellow_green(format!("Alignment of Demo: {align}")).println();
    }

}
