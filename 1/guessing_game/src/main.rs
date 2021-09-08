fn random_int() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let mut seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    seed ^= seed << 21;
    seed ^= seed >> 35;
    seed ^= seed << 4;
    seed
}

fn main() {
    let number = random_int() % 100 + 1;
    println!("Guess number between 1 and 100");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: u64 = input.trim().parse().unwrap();
        if input == number {
            println!("Correct!");
            break;
        }
        let message = if input > number { "Input is greater"} else { "Input is lesser" };
        println!("{}", message);
    }
}
