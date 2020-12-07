mod compiler;
mod instruction;
mod loader;
mod profiler;
mod simple_vm;
mod vm;

use crate::compiler::Compiler;
use crate::simple_vm::SimpleVm;
use loader::Loader;
use vm::Vm;

fn main() {
    let mut loader = Loader::new();
    if !loader.load() {
        return;
    }

    let use_compiler = true;
    if use_compiler {
        let mut compiler = Compiler::new();
        compiler.compile(loader.data());
        // compiler.dump_instructions();

        let mut vm = Vm::new();
        vm.setup(&compiler.instructions);
        vm.process(&compiler.instructions);
    // vm.output_profiling_result();
    } else {
        let mut vm = SimpleVm::new();
        vm.setup(loader.data());
        vm.process(loader.data());
        vm.output_profiling_result();
    }

    println!();
}
