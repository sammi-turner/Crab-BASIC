use crate::utils::*;
use std::collections::HashMap;

pub struct Runtime {
    pub idents: HashMap<String, String>,
    pub program: Vec<String>,
    pub gosub_stack: Vec<usize>,
    pub end_msg: String,
    pub current_line: usize,
    pub condition: bool,
}

impl Runtime {
    pub fn run_program(&mut self, f: &str) {
        seed();

        let result = read_from_file(&f);
        let r = line_count(&result);

        for i in 0..r {
            self.program.push(nth_line(&result, i).to_string());
        }

        let program_size = self.program.len();
        println!("");

        while self.current_line < program_size {
            let n = word_count(&self.program[self.current_line]);
            if n > 0 {
                let first = nth_word(&self.program[self.current_line], 0);
                match first {
                    "#" => (),
                    "if" => self.if_cmd(),
                    "cls" => self.cls_cmd(),
                    "let" => self.let_cmd(),
                    "end" => self.end_cmd(),
                    "goto" => self.goto_cmd(),
                    "gosub" => self.gosub_cmd(),
                    "input" => self.input_cmd(),
                    "print" => self.print_cmd(),
                    "iprint" => self.iprint_cmd(),
                    "return" => self.return_cmd(),
                    _ => self.assign_cmd(),
                }
            }
            self.current_line += 1;
        }

        if &self.end_msg != "" {
            println!("\n{}\n", &self.end_msg);
        } else {
            println!("\nOn line {}, the program ends.\n", self.program.len());
        }
    }
}
