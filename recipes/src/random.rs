use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn print_random_number() {
    let mut rng = rand::thread_rng();
    
    println!("Random f64: {}", &rng.gen::<f64>());
    //range:  The range start..end contains all values with start <= x < end. It is empty if start >= end.
    println!("Integer Between 0 and {:?}, {}", (0..10).end, &rng.gen_range(0..10));
    println!("Integer: {}", &rng.gen_range(0..10));
    println!("Float: {}", &rng.gen_range(0.0..10.0));

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    println!("Alphanumeric String: {}", rand_string);

    println!("Custom String: {}", get_custom_password());
}

fn get_custom_password() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789)(*&^%$#@!~";

    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(
            |_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            }
        ).collect();

    password
}