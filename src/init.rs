use clap::Parser;
use std::{env::current_dir, process::Command};

#[derive(Debug, Parser)]
#[command(about = "Initializes a Pillow project")]
pub struct Options {
    /// Name of your Pillow App
    #[arg(short, long)]
    pub app_name: String,

    /// Directory for init
    #[arg(short, long)]
    #[arg(default_value = "")]
    directory: String,
}

impl Options {
    pub fn init_project(&self) {
        self.cargo(vec!["new", &self.app_name]);

        self.cd();

        self.cargo(vec!["add", "pillow"]);
    }

    fn copy_templates(&self) {}

    fn cargo(&self, arg: Vec<&str>) {
        let mut cargo = Command::new("cargo");

        cargo.args(arg).status().expect("Fail in cargo");
    }

    fn cd(&self) {
        let path = format!(
            "{}/{}",
            current_dir().unwrap().display().to_string(),
            &self.app_name
        );

        Command::new("cd").arg(path).status().expect("Fail cd");
    }
}
