use std::{env, process};

fn main() {
    let task = env::args().nth(1).unwrap_or_default();
    let cmd = match task.as_str() {
        "lint" => "cargo fmt --check && cargo clippy -- -D warnings",
        "build" => "cargo build --release",
        "release" => "bunx relion -b Cargo.toml",
        _ => process::exit(1),
    };

    let status = process::Command::new("bash").arg("-c").arg(cmd).status();
    process::exit(status.map(|s| s.code().unwrap_or(1)).unwrap_or(1));
}
