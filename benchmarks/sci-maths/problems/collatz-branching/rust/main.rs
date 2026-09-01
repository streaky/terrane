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

fn stopping_steps(start: u64) -> u64 {
    let mut steps = 0;
    let mut value = start;
    while value != 1 {
        value = if value % 2 == 0 {
            value / 2
        } else {
            3 * value + 1
        };
        steps += 1;
    }
    steps
}

fn main() {
    let total: u64 = (1..=size_argument()).map(stopping_steps).sum();
    println!("{total}");
}
