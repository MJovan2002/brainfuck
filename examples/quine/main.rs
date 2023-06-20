use std::io::Write;
use std::str::FromStr;

use brainfuck::{
    program::Program,
    state::{OverflowStrategy, State},
};

fn main() {
    let program = Program::parse(include_str!("quine.bf")).unwrap();
    let _ = program.exec(
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
}
