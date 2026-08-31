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

fn generated_value(index: usize) -> f64 {
    let raw = (index % 200) as f64 - 100.0;
    0.01 * raw * raw + (index % 7) as f64 - 3.0
}

fn main() {
    let values: Vec<_> = (0..size_argument()).map(generated_value).collect();
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values
        .iter()
        .map(|value| (value - mean).powi(2))
        .sum::<f64>()
        / values.len() as f64;
    let outliers = values
        .iter()
        .filter(|value| (*value - mean).powi(2) > 2.5 * variance)
        .count();
    println!("{outliers}");
}
