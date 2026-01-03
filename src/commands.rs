// Copyright 2026 Adam Burucs. Licensed under custom Source Available License

pub fn print_info(cmd: clap::Command) {
    let about = cmd.get_about().map(|s| s.to_string()).unwrap_or_default();
    println!(
        "{} {} — {}",
        cmd.get_name(),
        cmd.get_version().unwrap_or(""),
        about
    );
    println!();
}
