use crate::opt::Opt;
use crate::profiler::Profiler;
use std::collections::HashMap;
use std::io;

const BF_MEMORY_BYTES: usize = 30000;

pub(crate) struct SimpleVm {
    instruction_pointer: usize,
    data_pointer: usize,
    bracket_depth: usize,
    memory: [u8; BF_MEMORY_BYTES],
    bracket_table: HashMap<usize, usize>,
    profiler: Profiler,
}

impl SimpleVm {
    pub(crate) fn new() -> Self {
        Self {
            instruction_pointer: 0,
            data_pointer: 0,
            bracket_depth: 0,
            memory: [0; BF_MEMORY_BYTES],
            bracket_table: HashMap::new(),
            profiler: Profiler::new(),
        }
    }

    pub(crate) fn setup(&mut self, program: &Vec<u8>, _opt: &Opt) {
        self.make_bracket_table(program);
    }

    pub(crate) fn process(&mut self, program: &Vec<u8>, opt: &Opt) {
        self.reset();

        while self.instruction_pointer < program.len() {
            match char::from(program[self.instruction_pointer] & 0xff) {
                '>' => {
                    self.profiler.rshift += 1;
                    self.data_pointer = self.data_pointer.wrapping_add(1);
                }
                '<' => {
                    self.profiler.lshift += 1;
                    self.data_pointer = self.data_pointer.wrapping_sub(1);
                }
                '+' => {
                    self.profiler.plus += 1;
                    self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_add(1)
                }
                '-' => {
                    self.profiler.minus += 1;
                    self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_sub(1)
                }
                '.' => {
                    self.profiler.dot += 1;
                    print!("{}", (self.memory[self.data_pointer] as char).to_string());
                }
                ',' => {
                    self.profiler.comma += 1;
                    self.memory[self.data_pointer] = self.read_1st_u8();
                }
                '[' => {
                    self.profiler.lbracket += 1;
                    self.process_left_bracket(program, opt);
                }
                ']' => {
                    self.profiler.rbracket += 1;
                    self.process_right_bracket(program, opt);
                }
                _ => { /* do nothing */ }
            }
            self.instruction_pointer = self.instruction_pointer.wrapping_add(1);
        }
    }

    pub(crate) fn output_profiling_result(&self, opt: &Opt) {
        if opt.use_profiler {
            self.profiler.output();
        }
    }

    fn read_1st_u8(&self) -> u8 {
        let mut word = String::new();
        io::stdin().read_line(&mut word).ok();
        word.trim().chars().nth(0).unwrap() as u8
    }

    fn process_left_bracket(&mut self, program: &Vec<u8>, opt: &Opt) {
        if opt.use_bracket_table {
            if self.memory[self.data_pointer] == 0 {
                self.instruction_pointer =
                    *self.bracket_table.get(&self.instruction_pointer).unwrap();
            }
        } else {
            if self.memory[self.data_pointer] != 0 {
                self.bracket_depth = self.bracket_depth.wrapping_add(1);
            } else {
                self.seek_right_bracket(program);
            }
        }
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

    fn process_right_bracket(&mut self, program: &Vec<u8>, opt: &Opt) {
        if opt.use_bracket_table {
            if self.memory[self.data_pointer] != 0 {
                self.instruction_pointer =
                    *self.bracket_table.get(&self.instruction_pointer).unwrap();
            }
        } else {
            if self.memory[self.data_pointer] != 0 {
                self.seek_left_bracket(program);
            } else {
                self.bracket_depth = self.bracket_depth.wrapping_sub(1);
            }
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

    fn make_bracket_table(&mut self, program: &Vec<u8>) -> bool {
        let mut left_brackets: Vec<usize> = Vec::new();
        let mut ip = 0;

        for ch in program {
            match *ch as char {
                '[' => {
                    left_brackets.push(ip);
                }
                ']' => {
                    let left_pos: usize = left_brackets.pop().unwrap();
                    let right_pos: usize = ip;
                    self.bracket_table.insert(right_pos, left_pos);
                    self.bracket_table.insert(left_pos, right_pos);
                }
                _ => { /* do nothing */ }
            }
            ip += 1;
        }
        left_brackets.len() == 0
    }
}
