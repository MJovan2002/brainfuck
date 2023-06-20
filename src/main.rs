use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;

use brainfuck::{
    program::Program,
    state::{OverflowStrategy, State},
};

fn main() {
    let mut s = String::new();
    File::open(std::env::args().nth(1).unwrap_or_else(|| {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    })).unwrap().read_to_string(&mut s).unwrap();
    let program = Program::parse(&s).unwrap();
    let s = program.exec(
        State::builder()
            .size(320)
            .overflow(OverflowStrategy::Wrap),
        || {
            print!("> ");
            std::io::stdout().flush().unwrap();
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            u8::from_str(s.trim_end()).unwrap()
        },
        |a| print!("{}", a as char),
    );
    println!("\n{s:?}");
}
