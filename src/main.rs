mod result;
mod shell;

use shell::Shell;

fn main() {
    Shell::new().run().unwrap_or_else(|e| e.exit());
}
