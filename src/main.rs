use std::{
    env,
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

fn main() {
    println!("Welcome to dead shell!");

    // TODO Parse config
    let stdin = io::stdin();

    'mainloop: loop {
        let cur_path = env::current_dir().unwrap();
        print!("{}\n> ", cur_path.display());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();

        let args = parse_input(input);
        if args.is_empty() {
            continue 'mainloop;
        }
        match args[0] {
            "exit" => break 'mainloop,
            "cd" => {
                let path = match args.get(1) {
                    Some(p) => p,
                    None => "~",
                };
                cd(path).unwrap_or_else(|_| println!("no such file or directory: {}", path))
            }
            _ => match exec(&args) {
                Ok(out) => println!("{}", String::from_utf8(out).unwrap()),
                Err(_) => println!("dead-shell: command not found: {}", args.join(" ")),
            },
        };
    }
}

fn parse_input(input: &str) -> Vec<&str> {
    let mut args: Vec<&str> = Vec::new();
    for arg in input.split(' ') {
        let arg = arg.trim();
        if arg.starts_with('#') {
            if let Some(last) = args.last() {
                if last.ends_with('\\') {
                    args.push(arg);
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            args.push(arg);
        }
    }
    args
}

fn cd(path: &str) -> io::Result<()> {
    let p = if path != "~" {
        PathBuf::from(path)
    } else {
        dirs::home_dir().unwrap()
    };
    env::set_current_dir(p.as_path())
}

fn exec(args: &[&str]) -> io::Result<Vec<u8>> {
    let cmd = Command::new(&args[0]).args(&args[1..]).output()?;

    Ok(Vec::from(cmd.stdout.as_slice()))
}
