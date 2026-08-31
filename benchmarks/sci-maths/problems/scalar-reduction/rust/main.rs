use std::{env, process};

fn size_argument() -> u64 {
    let mut arguments = env::args().skip(1);
    let size = arguments.next().and_then(|value| value.parse().ok());
    if arguments.next().is_some() || size.is_none_or(|value| value == 0) {
        eprintln!("expected exactly one positive integer size");
        process::exit(2);
    }
    size.unwrap()
}

fn main() {
    let mut total = 0_i64;
    for index in 0..size_argument() {
        let value = (index % 1_000) as i64 - 500;
        total += value * value;
    }
    println!("{total}");
}
