use std::fmt::{Debug, Formatter};

pub struct State {
    data: Vec<u8>,
    pos: usize,
    inc: fn(u8) -> u8,
    dec: fn(u8) -> u8,
}

impl State {
    fn new(size: usize, (inc, dec): (fn(u8) -> u8, fn(u8) -> u8)) -> Self {
        Self {
            data: vec![0; size],
            pos: 0,
            inc,
            dec,
        }
    }

    pub fn builder() -> StateBuilder {
        StateBuilder::new()
    }

    pub fn inc(&mut self) {
        self.data[self.pos] = (self.inc)(self.data[self.pos])
    }

    pub fn dec(&mut self) {
        self.data[self.pos] = (self.dec)(self.data[self.pos])
    }

    pub fn move_l(&mut self) {
        self.pos -= 1
    }

    pub fn move_r(&mut self) {
        self.pos += 1
    }

    pub fn get(&mut self) -> u8 {
        self.data[self.pos]
    }

    pub fn set(&mut self, t: u8) {
        self.data[self.pos] = t
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("data", &self.data)
            .field("pos", &self.pos)
            .finish_non_exhaustive()
    }
}

pub enum OverflowStrategy {
    Default,
    Panic,
    Wrap,
    Custom((fn(u8) -> u8, fn(u8) -> u8)),
}

impl OverflowStrategy {
    fn resolve(self) -> (fn(u8) -> u8, fn(u8) -> u8) {
        match self {
            OverflowStrategy::Default => (|a| a + 1, |a| a - 1),
            OverflowStrategy::Panic => (|a| a.checked_add(1).unwrap(), |a| a.checked_sub(1).unwrap()),
            OverflowStrategy::Wrap => (|a| a.wrapping_add(1), |a| a.wrapping_sub(1)),
            OverflowStrategy::Custom(f) => f,
        }
    }
}

pub struct StateBuilder {
    overflow_strategy: OverflowStrategy,
    size: usize,
}

impl StateBuilder {
    fn new() -> Self {
        Self {
            overflow_strategy: OverflowStrategy::Wrap,
            size: 30000,
        }
    }

    pub fn size(self, size: usize) -> Self {
        Self {
            size,
            ..self
        }
    }

    pub fn overflow(self, overflow_strategy: OverflowStrategy) -> Self {
        Self {
            overflow_strategy,
            ..self
        }
    }

    pub(crate) fn build(self) -> State {
        State::new(self.size, self.overflow_strategy.resolve())
    }
}
