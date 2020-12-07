use crate::instruction::Instruction;
use crate::opt::Opt;

pub(crate) struct Compiler {
    pub(crate) instructions: Vec<Instruction>,
}

impl Compiler {
    pub(crate) fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub(crate) fn compile(&mut self, program: &Vec<u8>, _opt: &Opt) {
        let mut prev_ch: char = '\0';
        let mut chars: usize = 0;

        for byte in program {
            let current_ch = *byte as char;
            match current_ch {
                '>' | '<' | '+' | '-' => {
                    if prev_ch == '\0' || prev_ch == current_ch {
                        prev_ch = current_ch;
                        chars += 1;
                    } else {
                        self.instructions.push(Instruction::new(prev_ch, chars));
                        prev_ch = current_ch;
                        chars = 1;
                    }
                }
                '.' | ',' | '[' | ']' => {
                    if prev_ch == '\0' {
                        self.instructions.push(Instruction::new(current_ch, 1));
                    } else {
                        self.instructions.push(Instruction::new(prev_ch, chars));
                        self.instructions.push(Instruction::new(current_ch, 1));
                        prev_ch = '\0';
                        chars = 0;
                    }
                }
                _ => { /* do nothing */ }
            }
        }
        if prev_ch != '\0' {
            self.instructions.push(Instruction::new(prev_ch, chars));
        }
    }

    pub(crate) fn dump_instructions(&self, opt: &Opt) {
        if opt.dump_instruction {
            for instruction in &self.instructions {
                if instruction.kind == '[' || instruction.kind == ']' {
                    println!("kind:{}, num:{}", instruction.kind, instruction.number);
                }
            }
        }
    }
}
