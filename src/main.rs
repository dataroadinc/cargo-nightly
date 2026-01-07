use std::env;
use std::process::{
    Command,
    Stdio,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Skip the first argument (program name) and the second argument (nightly)
    // Forward only the remaining arguments to cargo +nightly
    let cargo_args = &args[2..];

    // Build the command: cargo +nightly <args>
    let mut cmd = Command::new("cargo");
    cmd.arg("+nightly");
    cmd.args(cargo_args);

    // Forward stdin, stdout, stderr
    cmd.stdin(Stdio::inherit());
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());

    // Execute the command and exit with the same code
    let status = cmd.status().expect("Failed to execute cargo +nightly");
    std::process::exit(status.code().unwrap_or(1));
}
