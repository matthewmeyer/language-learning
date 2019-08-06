pub mod twelve_days {
    const DAYS: [(&str, &str); 4] = [
        ("Chicken in Road Ditch", "1st"), 
        ("Frogs a Leaping", "2nd"), 
        ("Maids a Milking", "3rd"),
        ("Golden Rings", "4th")
    ];

    pub fn twelve_days(day: usize, holiday: &str) {
        println!("On the {} day of {} my true love gave to me:", DAYS[day - 1].1, holiday);
        twelve_days_repeat(day)
    }

    pub fn twelve_days_song_until(day: usize, holiday: &str) {
        twelve_days_song_until_inner(day, holiday, 1)
    }

    fn twelve_days_song_until_inner(day: usize, holiday: &str, current: usize) {
        twelve_days(current, holiday);

        if day > current {
            println!("");
            twelve_days_song_until_inner(day, holiday, current + 1)
        }
    }

    fn twelve_days_repeat(day: usize) {
        println!("{} {}", day, DAYS[day - 1].0);
        if day > 1 {
            twelve_days_repeat(day - 1)
        }
    }
}