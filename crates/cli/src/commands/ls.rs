use crate::commands::command::Command;

pub struct Ls;

pub struct LsArgs {

}

impl Command for Ls {
    fn run(&self) -> Result<String, &'static str> {
        Ok(String::from("aaaa"))
    }
}
