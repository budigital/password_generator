use clap::Parser;
use rand::{thread_rng, Rng};

#[derive(Parser, Debug)]
#[command(version, about = "Simple password generator.", long_about = None)]
struct Args {
    #[arg(short, long)]
    uppercase: bool,
    #[arg(short, long)]
    symbol: bool,
    #[arg(short, long, default_value_t = 12)]
    length: u8,
}

fn main() {
    let args = Args::parse();

    generate_password(args);
}

fn generate_password(rules: Args) {
    let mut rng = thread_rng();

    let lowercase_chars = String::from("abcdefghijklmnopqrstuvwxyz");
    let uppercase_chars = lowercase_chars.to_uppercase();
    let number_chars = String::from("0123456789");
    let symbol_chars = String::from("!@#$%^&*()");

    let mut chars_vec: Vec<String> = Vec::new();

    if rules.uppercase {
        chars_vec.push(uppercase_chars);
    }

    if rules.symbol {
        chars_vec.push(symbol_chars);
    }

    chars_vec.push(format!("{}{}", lowercase_chars, number_chars));

    let chars = chars_vec.join("sep");

    let mut password = String::new();

    for _ in 0..rules.length {
        let index = rng.gen_range(0..chars.len());
        let char = chars.chars().nth(index).unwrap();

        password.push(char);
    }

    println!("{}", password)
}
