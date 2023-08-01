use std::process::Command;

pub fn init() {
    Command::new("cargo")
        .args(&[
            "generate",
            "--git",
            "https://github.com/SummaryPuppet/pillow-template",
        ])
        .spawn()
        .expect("Error generating")
        .wait()
        .expect("Error waiting for child");
}
