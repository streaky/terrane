use std::{env, process};

fn size_argument() -> usize {
    let mut arguments = env::args().skip(1);
    let size = arguments.next().and_then(|value| value.parse().ok());
    if arguments.next().is_some() || size.is_none_or(|value| value == 0) {
        eprintln!("expected exactly one positive integer size");
        process::exit(2);
    }
    size.unwrap()
}

fn transformed(index: usize) -> f64 {
    let value = (index % 1_000) as f64 / 100.0;
    let square = value * value;
    (square + 3.0 * value - 7.0) / (1.0 + square)
}

fn main() {
    println!("{}", (0..size_argument()).map(transformed).sum::<f64>());
}
