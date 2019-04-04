use std::io::{self, Write};

#[derive(PartialEq)]
enum Status {
    EXIT,
}

fn lsh_exec<'a, It>(args: It) -> Status
where
    It: IntoIterator<Item = &'a str>,
{
    Status::EXIT
}

fn lsh_loop() {
    loop {
        print!("> ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .unwrap_or_else(|err| panic!("unexpected error occurred reading from stdin: {}", err));

        if lsh_exec(line.split_whitespace()) == Status::EXIT {
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
