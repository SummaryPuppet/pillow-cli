use std::process::Command;

pub fn run() {
    Command::new("cargo-watch")
        .args(&["-x", "r"])
        .spawn()
        .expect("Error running cargo-watch")
        .wait()
        .expect("error running the command");
}
