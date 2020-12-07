#[derive(Default)]
pub(crate) struct Profiler {
    pub(crate) rshift: usize,
    pub(crate) lshift: usize,
    pub(crate) plus: usize,
    pub(crate) minus: usize,
    pub(crate) dot: usize,
    pub(crate) comma: usize,
    pub(crate) lbracket: usize,
    pub(crate) rbracket: usize,
}

impl Profiler {
    pub(crate) fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub(crate) fn output(&self) {
        println!("profile:");
        println!("  >: {}", self.rshift);
        println!("  <: {}", self.lshift);
        println!("  +: {}", self.plus);
        println!("  -: {}", self.minus);
        println!("  .: {}", self.dot);
        println!("  ,: {}", self.comma);
        println!("  [: {}", self.lbracket);
        println!("  ]: {}", self.rbracket);
    }
}
