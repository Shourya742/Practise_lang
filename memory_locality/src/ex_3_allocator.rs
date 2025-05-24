#[cfg(test)]
mod test {
    use r3bl_tui::set_jemalloc_in_main;

    #[test]
    fn test() {
        set_jemalloc_in_main!();
        println!("Jemalloca allocator is set.");
    }

}