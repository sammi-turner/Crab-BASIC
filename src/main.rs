mod runtime;
mod utils;

use runtime::Runtime;
use std::collections::HashMap;
use utils::file_exists;

fn main() {
    let mut rt = Runtime {
        idents: HashMap::new(),
        program: Vec::new(),
        end_msg: String::new(),
        current_line: 0,
        condition: true,
    };

    let args: Vec<String> = std::env::args().collect();
    let n = args.len();

    loop {
        if n == 1 {
            println!("A file name argument is required.");
            break;
        }

        if n > 2 {
            println!("Too many arguments. Only one is required.");
            break;
        }

        let script = &args[1];
        if file_exists(&script) {
            Runtime::run_program(&mut rt, &script);
            break;
        }

        println!("That file does not exist in the working directory.");
        break;
    }
}
