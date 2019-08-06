mod twelve_days;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let number = 3;

    println!("condition was {}", if number < 5 { true } else { false });

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }


    twelve_days::twelve_days::twelve_days_song_until(3, "Summer")

}
