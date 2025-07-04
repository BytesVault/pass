use rand::prelude::*;

fn generate_password() -> String {
    let mut rand = rand::rng();

    let mut password = "".to_string();

    const PASSWORD_CHAR_POOL: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                abcdefghijklmnopqrstuvwxyz\
                0123456789";

    for _ in 0..PASSWORD_CHAR_POOL.len() {
        let i = rand.random_range(0..PASSWORD_CHAR_POOL.len());

        password.push(PASSWORD_CHAR_POOL[i] as char);
    }

    return password;
}

fn main() {
    let password = generate_password();
    println!("{}", password);
}
