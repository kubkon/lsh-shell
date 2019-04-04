use nix::unistd;
use std::io::{self, Write};

#[derive(PartialEq)]
enum Status {
    Ok,
    Exit,
}

fn cd(args: &[&str]) -> Status {
    if args.len() > 1 {
        // handle errors...
    }
    unistd::chdir(args[0]).expect("couldn't change directory");

    Status::Ok
}

fn pwd() -> Status {
    let dir = unistd::getcwd().expect("cannot get present working directory");
    println!("{}", dir.to_str().expect("failed to convert to str"));
    io::stdout().flush().expect("failed to flush stdout");

    Status::Ok
}

fn lsh_exec(args: &[&str]) -> Status {
    if args.is_empty() {
        return Status::Ok;
    }

    let cmd = args[0];
    match cmd {
        "pwd" => pwd(),
        "cd" => cd(&args[1..]),
        "exit" => Status::Exit,
        _ => Status::Ok,
    }
}

fn lsh_loop() {
    loop {
        print!("> ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .unwrap_or_else(|err| panic!("unexpected error occurred reading from stdin: {}", err));

        let args: Vec<&str> = line.split_whitespace().collect();
        if lsh_exec(&args) == Status::Exit {
            break;
        }
    }
}

fn main() {
    // load config files
    // ...

    // run the command loop
    lsh_loop();

    // perform any shutdown/cleanup
}
