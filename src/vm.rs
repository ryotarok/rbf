use std::io;

const BF_MEMORY_BYTES: usize = 30000;

pub(crate) struct Vm {
    instruction_pointer: usize,
    data_pointer: usize,
    bracket_depth: usize,
    memory: [u8; BF_MEMORY_BYTES],
}

impl Vm {
    pub(crate) fn new() -> Self {
        Self {
            instruction_pointer: 0,
            data_pointer: 0,
            bracket_depth: 0,
            memory: [0; BF_MEMORY_BYTES],
        }
    }

    pub(crate) fn process(&mut self, program: &Vec<u8>) {
        self.reset();

        while self.instruction_pointer < program.len() {
            match char::from(program[self.instruction_pointer] & 0xff) {
                '>' => self.data_pointer = self.data_pointer.wrapping_add(1),
                '<' => self.data_pointer = self.data_pointer.wrapping_sub(1),
                '+' => {
                    self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_add(1)
                }
                '-' => {
                    self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_sub(1)
                }
                '.' => print!("{}", (self.memory[self.data_pointer] as char).to_string()),
                ',' => self.memory[self.data_pointer] = self.read_1st_u8(),
                '[' => {
                    if self.memory[self.data_pointer] != 0 {
                        self.bracket_depth = self.bracket_depth.wrapping_add(1);
                    } else {
                        self.seek_right_bracket(program);
                    }
                }
                ']' => {
                    if self.memory[self.data_pointer] != 0 {
                        self.seek_left_bracket(program);
                    } else {
                        self.bracket_depth = self.bracket_depth.wrapping_sub(1);
                    }
                }
                _ => { /* do nothing */ }
            }
            self.instruction_pointer = self.instruction_pointer.wrapping_add(1);
        }
    }

    fn read_1st_u8(&self) -> u8 {
        let mut word = String::new();
        io::stdin().read_line(&mut word).ok();
        word.trim().chars().nth(0).unwrap() as u8
    }

    fn seek_left_bracket(&mut self, program: &Vec<u8>) {
        let right_bracket_depth = self.bracket_depth;
        while self.instruction_pointer < program.len() {
            match char::from(program[self.instruction_pointer] & 0xff) {
                '[' => {
                    self.bracket_depth = self.bracket_depth.wrapping_add(1);
                    if self.bracket_depth == right_bracket_depth {
                        return;
                    }
                }
                ']' => self.bracket_depth = self.bracket_depth.wrapping_sub(1),
                _ => { /* do nothing */ }
            }
            self.instruction_pointer = self.instruction_pointer.wrapping_sub(1);
        }
    }

    fn seek_right_bracket(&mut self, program: &Vec<u8>) {
        let left_bracket_depth = self.bracket_depth;
        while (self.bracket_depth < usize::MAX) && (self.instruction_pointer < program.len()) {
            match char::from(program[self.instruction_pointer] & 0xff) {
                '[' => self.bracket_depth = self.bracket_depth.wrapping_add(1),
                ']' => {
                    self.bracket_depth = self.bracket_depth.wrapping_sub(1);
                    if self.bracket_depth == left_bracket_depth {
                        return;
                    }
                }
                _ => { /* do nothing */ }
            }
            self.instruction_pointer = self.instruction_pointer.wrapping_add(1);
        }
    }

    fn reset(&mut self) {
        self.instruction_pointer = 0;
        self.data_pointer = 0;
        self.bracket_depth = 0;
        for idx in 0..BF_MEMORY_BYTES {
            self.memory[idx] = 0;
        }
    }
}
