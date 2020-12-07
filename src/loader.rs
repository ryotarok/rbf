use crate::Opt;
use std::fs::File;
use std::io::Read;

pub(crate) struct Loader {
    data: Vec<u8>,
}

impl Loader {
    pub(crate) fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub(crate) fn load(&mut self, opt: &Opt) -> bool {
        if opt.filepath.as_str() == "" {
            return false;
        };
        self.data = Loader::load_from_file(opt.filepath.as_str());
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
