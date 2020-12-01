use std::env;
use std::fs::File;
use std::io::Read;

pub(crate) struct Loader {
    data: Vec<u8>,
}

impl Loader {
    pub(crate) fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub(crate) fn load(&mut self) -> bool {
        let args: Vec<String> = env::args().collect();
        let filepath = if args.len() > 1 {
            &args[1]
        } else {
            return false;
        };
        self.data = Loader::load_from_file(filepath);
        true
    }

    pub(crate) fn data(&self) -> &Vec<u8> {
        &self.data
    }

    fn load_from_file(filepath: &str) -> Vec<u8> {
        let mut file = File::open(filepath).unwrap();
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf).unwrap();
        buf
    }
}
