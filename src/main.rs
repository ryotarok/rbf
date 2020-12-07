mod compiler;
mod instruction;
mod loader;
mod opt;
mod profiler;
mod simple_vm;
mod vm;

use crate::compiler::Compiler;
use crate::opt::Opt;
use crate::simple_vm::SimpleVm;
use loader::Loader;
use vm::Vm;

fn main() {
    let opt = Opt::new();

    let mut loader = Loader::new();
    if !loader.load(&opt) {
        return;
    }

    if opt.use_compiler {
        let mut compiler = Compiler::new();
        compiler.compile(loader.data(), &opt);
        compiler.dump_instructions(&opt);

        let mut vm = Vm::new();
        vm.setup(&compiler.instructions, &opt);
        vm.process(&compiler.instructions, &opt);
        vm.output_profiling_result(&opt);
    } else {
        let mut vm = SimpleVm::new();
        vm.setup(loader.data(), &opt);
        vm.process(loader.data(), &opt);
        vm.output_profiling_result(&opt);
    }

    println!();
}
