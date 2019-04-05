use std::ffi;
use std::io::{self, Write};

use super::result::{Error, Result};

pub struct Shell;

impl Shell {
    pub fn new() -> Self {
        Self
    }

    pub fn run(self) -> Result {
        loop {
            print!("> ");
            io::stdout().flush()?;

            let mut line = String::new();
            io::stdin().read_line(&mut line)?;

            let args: Vec<&str> = line.split_whitespace().collect();
            if args.is_empty() {
                continue;
            }

            match args[0] {
                "cd" => Self::cd(&args[1..])?,
                "exit" => return Ok(()),
                _ => Self::cmd(&args)?,
            }
        }
    }

    fn cmd(args: &[&str]) -> Result {
        match nix::unistd::fork() {
            Ok(nix::unistd::ForkResult::Parent { child, .. }) => {
                while let Ok(wait_status) =
                    nix::sys::wait::waitpid(child, Some(nix::sys::wait::WaitPidFlag::WUNTRACED))
                {
                    match wait_status {
                        nix::sys::wait::WaitStatus::Exited(pid, status) => break,
                        nix::sys::wait::WaitStatus::Signaled(pid, signal, case) => break,
                        _ => {}
                    }
                }

                Ok(())
            }
            Ok(nix::unistd::ForkResult::Child) => {
                let mut p = std::path::PathBuf::from("/bin");
                p.push(args[0]);
                let cmd = ffi::CString::new(p.to_str().unwrap()).unwrap();
                let mut cmd_args = vec![cmd.clone()];
                for arg in &args[1..] {
                    cmd_args.push(ffi::CString::new(*arg).unwrap());
                }
                nix::unistd::execvp(&cmd, &cmd_args).unwrap();

                Ok(())
            }
            Err(_) => Ok(()),
        }
    }

    fn cd(args: &[&str]) -> Result {
        if args.len() > 1 {
            eprintln!("Too many args for cd");
            return Ok(());
        }

        match nix::unistd::chdir(args[0]) {
            _ => Ok(()),
        }
    }
}
