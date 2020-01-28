use rox_lib::vm::{InterpretResult, VM};
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process;

fn main() {
    let mut vm = VM::new();

    if env::args().len() == 1 {
        repl(&mut vm);
    } else if env::args().len() == 2 {
        run_file(&mut vm, &env::args().collect::<Vec<_>>()[1]);
    } else {
        eprintln!("Usage: rox [path]");
        process::exit(64);
    }
}

fn repl(vm: &mut VM) -> ! {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        vm.interpret(&line);
    }
}

fn run_file<P: AsRef<Path>>(vm: &mut VM, path: P) {
    let source = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Could not open file '{}'.\n{}", path.as_ref().display(), e);
            process::exit(74);
        }
    };

    let result = vm.interpret(&source);
    match result {
        InterpretResult::CompileError => process::exit(65),
        InterpretResult::RuntimeError => process::exit(70),
        InterpretResult::Ok => process::exit(0),
    }
}
