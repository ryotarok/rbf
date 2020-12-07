pub(crate) struct Opt {
    pub(crate) use_compiler: bool,
    pub(crate) dump_instruction: bool,
    pub(crate) use_profiler: bool,
    pub(crate) use_bracket_table: bool,
    pub(crate) filepath: String,
}

impl Opt {
    pub(crate) fn new() -> Self {
        let mut use_compiler = false;
        let mut dump_instruction = false;
        let mut use_profiler = false;
        let mut use_bracket_table = false;
        let mut filepath = String::new();

        for arg in std::env::args() {
            match arg.as_str() {
                "--use-compiler" => use_compiler = true,
                "--dump-instruction" => dump_instruction = true,
                "--use-profiler" => use_profiler = true,
                "--use-bracket-table" => use_bracket_table = true,
                _ => filepath = arg.to_string(),
            }
        }

        Self {
            use_compiler,
            dump_instruction,
            use_profiler,
            use_bracket_table,
            filepath,
        }
    }
}
