pub(crate) struct Instruction {
    pub(crate) kind: char,
    pub(crate) number: usize,
}

impl Instruction {
    pub(crate) fn new(kind: char, number: usize) -> Self {
        Self { kind, number }
    }
}
