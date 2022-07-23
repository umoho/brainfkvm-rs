use std::{fs::File, io::Read, env, process::ExitCode};

const VERSION: &str = "0.1";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Need a file");
        return ExitCode::FAILURE;
    }
    let filepath = match args[1].as_str() {
        "--help" => {
            println!("Simple Brainfuck Virtual Machine (version {}, Rust impl)", VERSION);
            println!("Usage:\n\t{0} <FILEPATH>\n\t{0} --help", args[0]);
            return ExitCode::SUCCESS;
        },
        fp => fp
    };

    let mut source_file = match File::open(filepath) {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            return ExitCode::FAILURE;
        },
    };

    let mut code = String::new();
    source_file.read_to_string(&mut code).expect("Cannot read to string");

    exec(&code);

    ExitCode::SUCCESS
}

#[derive(Debug)]
struct VirtualMachine {
    cells: Vec<usize>,
    pointer: usize,
}

impl VirtualMachine {
    fn new(cells_size: usize) -> Self {
        Self {
            cells: vec![0; cells_size],
            pointer: cells_size / 2 - 1
         }
    }
}

fn exec(code: &str) {
    let mut vm = VirtualMachine::new(1024);
    let mut jump_positions: Vec<usize> = vec![];

    let code_bytes = code.as_bytes();

    let mut position = 0;
    loop {  // repeat
        let cmd = code_bytes[position];
        match cmd as char {
            '+' => { vm.cells[vm.pointer] = vm.cells[vm.pointer] + 1; },
            '-' => { vm.cells[vm.pointer] = vm.cells[vm.pointer] - 1; },
            '>' => { vm.pointer = vm.pointer + 1; },
            '<' => { vm.pointer = vm.pointer - 1; },
            '.' => { print!("{}", char::from_u32(vm.cells[vm.pointer] as u32).expect("ERROR: Not a char")); },
            ',' => { todo!(); },
            '[' => { jump_positions.push(position); },
            ']' => {
                if vm.cells[vm.pointer] != 0 {
                    position = *jump_positions.last().expect("ERROR: Unpaired `]`");
                }
            },
            _ => { /* ignore */ }
        }

        position = position + 1;
        if position > code_bytes.len() - 1 {  // until
            println!();
            break;
        }
    }
}
