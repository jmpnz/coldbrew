use coldbrew::jvm::{read_class_file, JVMParser};
use std::path::Path;

fn main() {
    // What are the program components ?
    // 1. Reads and parse Java class file.
    let path = Path::new("./support/SingleFuncCall.class");
    let class_file_bytes = read_class_file(path);
    let class_file = JVMParser::parse(&class_file_bytes)
        .expect("JVMParser failed with some error");
    // 2. Passes bytecode to an Interpreter class
    // 3. Interepreter executes bytecode and records a trace.
    // 4. When trace is hot it is compiled to assembly
    // 5. Interpreter/Handler executes assembly and returns value
    // 6. Repeat
    println!("{class_file:?}");
}
