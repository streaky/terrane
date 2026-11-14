use std::{env, os::unix::process::CommandExt, process::Command};

fn main() {
    let mut arguments = env::args_os().skip(1);
    let binary = arguments.next().expect("benchmark executable is required");
    let error = Command::new(binary)
        .arg0("statement-owner-workload")
        .args(arguments)
        .exec();
    panic!("cannot launch benchmark: {error}");
}
