use std::collections::HashMap;
use crate::commands::{self, Command};
// use crate::commands::Command;

pub struct Shell {
    commands: HashMap<String, Box<dyn Command>>
}

impl Shell {
    pub fn new() -> Result<Shell, &'static str> {
        let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();
        commands.insert(String::from("ls"), Box::new(commands::Ls{}));
        Ok(Shell{commands})
    }

    pub fn run(&self, command: &str) {
        println!("shell");
    }
}
