use std::process::{Command, ExitStatus};

fn main() {
    let command = std::env::args().nth(1);
    if command.as_deref() != Some("validate") {
        eprintln!("usage: cargo xtask validate");
        std::process::exit(2);
    }

    let checks: &[(&str, &[&str])] = &[
        ("cargo", &["metadata", "--format-version", "1", "--locked"]),
        ("cargo", &["fmt", "--all", "--", "--check"]),
        ("cargo", &["test", "--workspace", "--locked"]),
        (
            "cargo",
            &[
                "clippy",
                "--workspace",
                "--all-targets",
                "--locked",
                "--",
                "-D",
                "warnings",
            ],
        ),
        ("cargo", &["doc", "--workspace", "--no-deps", "--locked"]),
        (
            "cargo",
            &["+1.93.0", "test", "--workspace", "--locked"],
        ),
    ];

    for (program, arguments) in checks {
        println!("running: {program} {}", arguments.join(" "));
        match run(program, arguments) {
            Ok(status) if status.success() => {}
            Ok(status) => {
                eprintln!("validation command failed with {status}");
                std::process::exit(status.code().unwrap_or(1));
            }
            Err(error) => {
                eprintln!("failed to execute {program}: {error}");
                std::process::exit(1);
            }
        }
    }
}

fn run(program: &str, arguments: &[&str]) -> std::io::Result<ExitStatus> {
    Command::new(program).args(arguments).status()
}
