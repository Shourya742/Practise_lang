#[cfg(test)]
mod test {
    use std::time::{Duration, Instant};

    use chrono::NaiveDate;
    extern crate chrono;
    #[test]
    pub fn test_stdtime() {
        let dur1 = Duration::from_secs(15);
        println!("{}", dur1.as_secs());

        let dur2 = Duration::from_millis(15500);
        let dur3 = dur1.checked_sub(dur2);
        println!("{}", dur3.unwrap_or_default().as_millis());

        let now = Instant::now();
        std::thread::sleep(Duration::from_millis(200));
        println!("{}", now.elapsed().as_millis());
    }

    #[test]
    fn test_chrono() {
        let utc_now = chrono::Utc::now();
        println!("{}", utc_now.format("%H %Y %b %d"));

        let local_time = chrono::Local::now();
        println!("{}", local_time.format("%Y %b %d %H %Z"));

        let date = NaiveDate::from_isoywd_opt(2024, 1, chrono::Weekday::Mon);
        let unwrapped_date = date.unwrap();
        println!("{}", unwrapped_date.format("Day of year is: %j"));
        unwrapped_date
            .iter_days()
            .take(4)
            .for_each(|d| println!("{}", d.format("%j")));

        let date2 = NaiveDate::from_yo_opt(2024, 366);
        println!("{}", date2.unwrap().format("%A %B %d"));

        let birthday = NaiveDate::parse_from_str("2024|||09||07", "%Y|||%m||%d");
        println!("{}", birthday.unwrap());
    }
}
