mod loader;
mod profiler;
mod vm;

use loader::Loader;
use vm::Vm;

fn main() {
    let mut loader = Loader::new();
    if !loader.load() {
        return;
    }

    let mut vm = Vm::new();
    vm.setup(loader.data());
    vm.process(loader.data());
    vm.output_profiling_result();

    println!();
}
