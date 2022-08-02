use crate::utils::*;
use std::collections::HashMap;

pub struct Runtime {
    pub idents: HashMap<String, String>,
    pub program: Vec<String>,
    pub end_msg: String,
    pub current_line: usize,
    pub condition: bool,
}

impl Runtime {
    pub fn run_program(&mut self, f: &str) {
        // Clear screen and seed random numbers
        seed();

        // Set default values in Runtime
        self.idents.clear();
        self.program.clear();
        self.end_msg.clear();

        // Read program from file in the program vector
        let result = read_from_file(&f);
        let r = line_count(&result);

        for i in 0..r {
            self.program.push(nth_line(&result, i).to_string());
        }

        // Determine program size
        let program_size = self.program.len();

        // If there are no subroutine syntax errors, the program runs
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
                    "input" => self.input_cmd(),
                    "print" => self.print_cmd(),
                    "iprint" => self.iprint_cmd(),
                    _ => self.assign_cmd(),
                }
            }
            self.current_line += 1;
        }

        if &self.end_msg != "" {
            println!("\n{}\n", &self.end_msg);
        } else {
            println!("\nOn line {}, the program ends.", self.program.len());
        }
    }

    fn valid_name(&mut self, x: &str) -> bool {
        let first_char = nth_char(&x, 0);
        let lower = first_char.is_ascii_lowercase();
        let reserved = vec![
            "#", "if", "or", "and", "cls", "let", "end", "goto", "true", "false", "input", "print",
            "iprint",
        ];
        let is_reserved = reserved.contains(&x);
        return lower && !is_reserved;
    }

    fn if_cmd(&mut self) {
        loop {
            if self.condition == false {
                break;
            }

            let s = remove_nth_word(&self.program[self.current_line], 0);

            if word_count(&s) != 3 {
                self.end_msg = format!(
                    "On line {}, there is a syntax error.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            let left: String;
            let right: String;

            let word1 = nth_word(&s, 0);
            if self.idents.contains_key(word1) {
                left = self.idents[word1].clone();
            } else {
                left = word1.to_string();
            }

            let word3 = nth_word(&s, 2);
            if self.idents.contains_key(word3) {
                right = self.idents[word3].clone();
            } else {
                right = word3.to_string();
            }

            let word2 = nth_word(&s, 1);
            match word2 {
                "==" => self.parse_equal(left, right),
                "!=" => self.parse_unequal(left, right),
                ">" => self.parse_gt(left, right),
                "<" => self.parse_lt(left, right),
                ">=" => self.parse_gte(left, right),
                "<=" => self.parse_lte(left, right),
                "and" => self.parse_and(left, right),
                "or" => self.parse_or(left, right),
                _ => self.operator_error(),
            }
            break;
        }
    }

    fn parse_equal(&mut self, x: String, y: String) {
        if x != y {
            self.condition = false;
        }
    }

    fn parse_unequal(&mut self, x: String, y: String) {
        if x == y {
            self.condition = false;
        }
    }

    fn parse_gt(&mut self, x: String, y: String) {
        loop {
            if !is_int(&x) || !is_int(&y) {
                self.condition = false;
                break;
            }

            let vx = to_int(&x);
            let vy = to_int(&y);

            if vx <= vy {
                self.condition = false;
                break;
            }

            break;
        }
    }

    fn parse_lt(&mut self, x: String, y: String) {
        loop {
            if !is_int(&x) || !is_int(&y) {
                self.condition = false;
                break;
            }

            let vx = to_int(&x);
            let vy = to_int(&y);

            if vx >= vy {
                self.condition = false;
                break;
            }

            break;
        }
    }

    fn parse_gte(&mut self, x: String, y: String) {
        loop {
            if !is_int(&x) || !is_int(&y) {
                self.condition = false;
                break;
            }

            let vx = to_int(&x);
            let vy = to_int(&y);

            if vx < vy {
                self.condition = false;
                break;
            }

            break;
        }
    }

    fn parse_lte(&mut self, x: String, y: String) {
        loop {
            if !is_int(&x) || !is_int(&y) {
                self.condition = false;
                break;
            }

            let vx = to_int(&x);
            let vy = to_int(&y);

            if vx > vy {
                self.condition = false;
                break;
            }

            break;
        }
    }

    fn parse_and(&mut self, x: String, y: String) {
        if x != "true" || y != "true" {
            self.condition = false;
        }
    }

    fn parse_or(&mut self, x: String, y: String) {
        loop {
            if x != "true" && x != "false" {
                self.condition = false;
                break;
            }

            if y != "true" && y != "false" {
                self.condition = false;
                break;
            }

            if x == "false" && y == "false" {
                self.condition = false;
                break;
            }

            break;
        }
    }

    fn operator_error(&mut self) {
        self.end_msg = format!(
            "On line {}, the operator is invalid.",
            self.current_line + 1
        );
        self.current_line = self.program.len();
    }

    fn cls_cmd(&mut self) {
        loop {
            if self.condition == false {
                break;
            }

            if &self.program[self.current_line] != "cls" {
                self.end_msg = format!(
                    "On line {}, there is a syntax error.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
            }

            cls();
            break ();
        }
    }

    fn let_cmd(&mut self) {
        loop {
            if self.condition == false {
                break;
            }

            let s = remove_nth_word(&self.program[self.current_line], 0);
            let n = word_count(&s);

            if n != 3 {
                self.end_msg = format!(
                    "On line {}, there is a syntax error.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            if nth_word(&s, 1) != "=" {
                self.end_msg = format!(
                    "On line {}, there is a syntax error.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            let left = nth_word(&s, 0);
            if !self.valid_name(left) {
                break;
            }

            if self.idents.contains_key(left) {
                self.end_msg = format!(
                    "On line {}, that identifier is already defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            self.idents.remove(left);
            let right = nth_word(&s, 2);

            if self.idents.contains_key(right) {
                self.idents
                    .insert(left.to_string(), self.idents[right].clone());
                break;
            }

            self.idents.insert(left.to_string(), right.to_string());
            break;
        }
    }

    fn end_cmd(&mut self) {
        loop {
            let s = self.program[self.current_line].as_str();

            if s == "end if" && self.condition == false {
                self.condition = true;
                break;
            }

            if s == "end if" && self.condition == true {
                break;
            }

            if s == "end" && self.condition == true {
                self.end_msg = format!("On line {}, the program ends.", self.current_line + 1);
                self.current_line = self.program.len();
                break;
            }

            if s == "end" && self.condition == false {
                break;
            }

            self.end_msg = format!(
                "On line {}, there is a syntax error.",
                self.current_line + 1
            );
            self.current_line = self.program.len();
            break;
        }
    }

    fn goto_cmd(&mut self) {
        loop {
            if self.condition == false {
                break;
            }

            let n = word_count(&self.program[self.current_line]);
            if n != 2 {
                self.end_msg = format!(
                    "On line {}, there is a syntax error.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            let s = nth_word(&self.program[self.current_line], 1);
            if is_int(s) {
                self.current_line = (to_int(s) - 2) as usize;
                break;
            }

            if !self.idents.contains_key(s) {
                self.end_msg = format!(
                    "On line {}, that identifier is not defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            self.current_line = (to_int(&self.idents[s]) - 2) as usize;
            break;
        }
    }

    fn input_cmd(&mut self) {
        loop {
            if self.condition == false {
                break;
            }

            let s = remove_nth_word(&self.program[self.current_line], 0);
            let n = word_count(&s);

            if n != 1 {
                self.end_msg = format!(
                    "On line {}, there is a syntax error.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            if !self.idents.contains_key(&s) {
                self.end_msg = format!(
                    "On line {}, that identifier is not previously defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            let value = input("");
            self.idents.remove(&s);
            self.idents.insert(s, value);
            break;
        }
    }

    fn print_cmd(&mut self) {
        if self.condition == true {
            let s = remove_nth_word(&self.program[self.current_line], 0);
            println!("{}", &s);
        }
    }

    fn iprint_cmd(&mut self) {
        loop {
            if self.condition == false {
                break;
            }

            let s = remove_nth_word(&self.program[self.current_line], 0);
            let n = word_count(&s);
            let mut output = String::new();

            for i in 0..n {
                let word = nth_word(&s, i);
                if self.idents.contains_key(word) {
                    output.push_str(&self.idents[word]);
                    output.push(' ');
                } else {
                    output.push_str(word);
                    output.push(' ');
                }
            }

            output.pop();
            println!("{}", &output);
            break;
        }
    }

    fn assign_cmd(&mut self) {
        loop {
            if self.condition == false {
                break;
            }

            let s = self.program[self.current_line].clone();
            let n = word_count(&s);

            if n != 3 {
                self.end_msg = format!(
                    "On line {}, there is a syntax error.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            let a = nth_word(&s, 1);
            let left = nth_word(&s, 0);
            let right = nth_word(&s, 2);

            match a {
                "=" => self.reassign(left, right),
                "+=" => self.plus_equal(left, right),
                "-=" => self.minus_equal(left, right),
                "*=" => self.times_equal(left, right),
                "/=" => self.divide_equal(left, right),
                "%=" => self.mod_equal(left, right),
                "?=" => self.rand_equal(left, right),
                _ => self.assign_error(),
            }
            break;
        }
    }

    fn reassign(&mut self, x: &str, y: &str) {
        loop {
            if self.condition == false {
                break;
            }

            if !self.idents.contains_key(x) {
                self.end_msg = format!(
                    "On line {}, that subroutine is not defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            self.idents.remove(x);
            if self.idents.contains_key(y) {
                self.idents.insert(x.to_string(), self.idents[y].clone());
            } else {
                self.idents.insert(x.to_string(), y.to_string());
            }
            break;
        }
    }

    fn plus_equal(&mut self, x: &str, y: &str) {
        loop {
            if self.condition == false {
                break;
            }

            if !self.idents.contains_key(x) {
                self.end_msg = format!(
                    "On line {}, that identifier is not previously defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            if !is_int(&self.idents[x]) {
                self.end_msg = format!(
                    "On line {}, that assignment is not defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            let mut total = to_int(&self.idents[x]);

            if is_int(y) {
                total += to_int(y);
                self.idents.remove(x);
                self.idents.insert(x.to_string(), total.to_string());
                break;
            }

            if !self.idents.contains_key(y) {
                self.end_msg = format!(
                    "On line {}, that assignment is not defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            total += to_int(&self.idents[y]);
            self.idents.remove(x);
            self.idents.insert(x.to_string(), total.to_string());
            break;
        }
    }

    fn minus_equal(&mut self, x: &str, y: &str) {
        loop {
            if self.condition == false {
                break;
            }

            if !self.idents.contains_key(x) {
                self.end_msg = format!(
                    "On line {}, that identifier is not previously defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            if !is_int(&self.idents[x]) {
                self.end_msg = format!(
                    "On line {}, that assignment is not defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            let mut total = to_int(&self.idents[x]);

            if is_int(y) {
                total -= to_int(y);
                self.idents.remove(x);
                self.idents.insert(x.to_string(), total.to_string());
                break;
            }

            if !self.idents.contains_key(y) {
                self.end_msg = format!(
                    "On line {}, that assignment is not defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            total += to_int(&self.idents[y]);
            self.idents.remove(x);
            self.idents.insert(x.to_string(), total.to_string());
            break;
        }
    }

    fn times_equal(&mut self, x: &str, y: &str) {
        loop {
            if self.condition == false {
                break;
            }

            if !self.idents.contains_key(x) {
                self.end_msg = format!(
                    "On line {}, that identifier is not previously defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            if !is_int(&self.idents[x]) {
                self.end_msg = format!(
                    "On line {}, that assignment is not defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            let mut total = to_int(&self.idents[x]);

            if is_int(y) {
                total *= to_int(y);
                self.idents.remove(x);
                self.idents.insert(x.to_string(), total.to_string());
                break;
            }

            if !self.idents.contains_key(y) {
                self.end_msg = format!(
                    "On line {}, that assignment is not defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            total += to_int(&self.idents[y]);
            self.idents.remove(x);
            self.idents.insert(x.to_string(), total.to_string());
            break;
        }
    }

    fn divide_equal(&mut self, x: &str, y: &str) {
        loop {
            if self.condition == false {
                break;
            }

            if !self.idents.contains_key(x) {
                self.end_msg = format!(
                    "On line {}, that identifier is not previously defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            if !is_int(&self.idents[x]) {
                self.end_msg = format!(
                    "On line {}, that assignment is not defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            let mut total = to_int(&self.idents[x]);

            if is_int(y) {
                total /= to_int(y);
                self.idents.remove(x);
                self.idents.insert(x.to_string(), total.to_string());
                break;
            }

            if !self.idents.contains_key(y) {
                self.end_msg = format!(
                    "On line {}, that assignment is not defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            total += to_int(&self.idents[y]);
            self.idents.remove(x);
            self.idents.insert(x.to_string(), total.to_string());
            break;
        }
    }

    fn mod_equal(&mut self, x: &str, y: &str) {
        loop {
            if self.condition == false {
                break;
            }

            if !self.idents.contains_key(x) {
                self.end_msg = format!(
                    "On line {}, that identifier is not previously defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            if !is_int(&self.idents[x]) {
                self.end_msg = format!(
                    "On line {}, that assignment is not defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            let mut total = to_int(&self.idents[x]);

            if is_int(y) {
                total %= to_int(y);
                self.idents.remove(x);
                self.idents.insert(x.to_string(), total.to_string());
                break;
            }

            if !self.idents.contains_key(y) {
                self.end_msg = format!(
                    "On line {}, that assignment is not defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            total += to_int(&self.idents[y]);
            self.idents.remove(x);
            self.idents.insert(x.to_string(), total.to_string());
            break;
        }
    }

    fn rand_equal(&mut self, x: &str, y: &str) {
        loop {
            if self.condition == false {
                break;
            }

            if !self.idents.contains_key(x) {
                self.end_msg = format!(
                    "On line {}, that identifier is not previously defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            if !is_int(&self.idents[x]) {
                self.end_msg = format!(
                    "On line {}, that assignment is not defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            let mut total = to_int(&self.idents[x]);

            if is_int(y) {
                total += pseudo(0, to_int(y));
                self.idents.remove(x);
                self.idents.insert(x.to_string(), total.to_string());
                break;
            }

            if !self.idents.contains_key(y) {
                self.end_msg = format!(
                    "On line {}, that assignment is not defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            total += to_int(&self.idents[y]);
            self.idents.remove(x);
            self.idents.insert(x.to_string(), total.to_string());
            break;
        }
    }

    fn assign_error(&mut self) {
        self.end_msg = format!("On line {}, that token is not defined.", self.current_line);
        self.current_line = self.program.len();
    }
}
