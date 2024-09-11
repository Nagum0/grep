use grep::Grep;
use std::{env, process::exit};

fn main() {
    let grep = match Grep::from(env::args()) {
        Ok(grep) => grep,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };
    grep.grep();
}
