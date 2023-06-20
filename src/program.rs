use crate::state::{State, StateBuilder};

pub struct Program {
    ops: Vec<Op>,
}

impl Program {
    pub fn parse(s: &str) -> Option<Self> {
        let mut stack = Vec::new();
        stack.push(vec![]);
        for c in s.chars() {
            match Op::from(c) {
                Ok(op) => match stack.last_mut() {
                    Some(top) => top.push(op),
                    None => return None,
                },
                Err(_) => match c {
                    '[' => stack.push(vec![]),
                    ']' => {
                        let t = match stack.pop() {
                            None => return None,
                            Some(t) => t
                        };
                        let t = Program {
                            ops: t,
                        };
                        match stack.last_mut() {
                            Some(top) => top.push(Op::Loop(t)),
                            None => return None,
                        }
                    }
                    _ => {}
                }
            }
        }
        let program = stack.pop();
        let empty = stack.pop();
        match (program, empty) {
            (Some(ops), None) => Some(Program { ops }),
            _ => None
        }
    }

    pub fn exec<R: Fn() -> u8, W: FnMut(u8)>(&self, state_builder: StateBuilder, read: R, mut write: W) -> State {
        let mut state = state_builder.build();
        self.exec_with(&mut state, &read, &mut write);
        state
    }

    fn exec_with<F: Fn() -> u8, W: FnMut(u8)>(&self, s: &mut State, read: &F, write: &mut W) {
        for op in &self.ops {
            op.exec(s, read, write);
        }
    }
}

pub enum Op {
    Inc,
    Dec,
    MoveL,
    MoveR,
    Read,
    Write,
    Loop(Program),
}

impl Op {
    fn from(c: char) -> Result<Self, ()> {
        match c {
            '+' => Ok(Op::Inc),
            '-' => Ok(Op::Dec),
            '<' => Ok(Op::MoveL),
            '>' => Ok(Op::MoveR),
            ',' => Ok(Op::Read),
            '.' => Ok(Op::Write),
            _ => Err(()),
        }
    }

    fn exec<F: Fn() -> u8, W: FnMut(u8)>(&self, s: &mut State, read: &F, write: &mut W) {
        match &self {
            Op::Inc => s.inc(),
            Op::Dec => s.dec(),
            Op::MoveL => s.move_l(),
            Op::MoveR => s.move_r(),
            Op::Read => s.set(read()),
            Op::Write => write(s.get()),
            Op::Loop(p) => while s.get() != 0 {
                p.exec_with(s, read, write)
            },
        }
    }
}
