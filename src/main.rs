use nix::unistd;
use std::io::{self, Write};

#[derive(PartialEq)]
enum Status {
    Ok,
    Exit,
}

fn pwd() -> Status {
    let dir = unistd::getcwd().expect("cannot get present working directory");
    println!("{}", dir.to_str().expect("failed to convert to str"));
    io::stdout().flush().expect("failed to flush stdout");

    Status::Ok
}

fn lsh_exec<'a, It>(args: It) -> Status
where
    It: IntoIterator<Item = &'a str>,
{
    let args: Vec<&str> = args.into_iter().collect();
    if args.is_empty() {
        return Status::Ok;
    }

    let cmd = args[0];
    match cmd {
        "pwd" => pwd(),
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

        if lsh_exec(line.split_whitespace()) == Status::Exit {
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
