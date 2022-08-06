use crate::utils::*;
use crate::Runtime;

impl Runtime {
    pub fn if_cmd(&mut self) {
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

    pub fn cls_cmd(&mut self) {
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

    pub fn let_cmd(&mut self) {
        loop {
            if self.condition == false {
                break;
            }

            let s = remove_nth_word(&self.program[self.current_line], 0);
            let n = word_count(&s);

            if n < 3 {
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

            let s1 = remove_nth_word(&s, 0);
            if nth_word(&s1, 0) != "=" {
                self.end_msg = format!(
                    "On line {}, there is a syntax error.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
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
            let right = remove_nth_word(&s1, 0);

            if self.idents.contains_key(&right) {
                self.idents
                    .insert(left.to_string(), self.idents[&right].clone());
                break;
            }

            self.idents.insert(left.to_string(), right.to_string());
            break;
        }
    }

    pub fn end_cmd(&mut self) {
        loop {
            if self.condition == false {
                break;
            }

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

    pub fn goto_cmd(&mut self) {
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

    pub fn gosub_cmd(&mut self) {
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
                self.gosub_stack.push(self.current_line);
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

            self.gosub_stack.push(self.current_line);
            self.current_line = (to_int(&self.idents[s]) - 2) as usize;
            break;
        }
    }

    pub fn input_cmd(&mut self) {
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

    pub fn print_cmd(&mut self) {
        loop {
            if self.condition == false {
                break;
            }

            let s = remove_nth_word(&self.program[self.current_line], 0);
            println!("{}", &s);
            break;
        }
    }

    pub fn iprint_cmd(&mut self) {
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

    pub fn return_cmd(&mut self) {
        loop {
            if self.condition == false {
                break;
            }

            let s = self.program[self.current_line].as_str();
            let n = self.gosub_stack.len();

            if n == 0 {
                self.end_msg = format!(
                    "On line {}, there is a return command without a gosub call.",
                    self.current_line + 1
                );
                break;
            }

            if s == "return" {
                self.current_line = self.gosub_stack[n - 1];
                self.gosub_stack.pop();
                break;
            }

            self.end_msg = format!(
                "On line {}, there is a syntax error.",
                self.current_line + 1
            );
            break;
        }
    }

    pub fn assign_cmd(&mut self) {
        loop {
            if self.condition == false {
                break;
            }

            let s = self.program[self.current_line].clone();
            let n = word_count(&s);

            if n > 2 {
                let name = nth_word(&s, 0);
                let assign = nth_word(&s, 1);

                let s1 = remove_nth_word(&s, 0);
                let s2 = remove_nth_word(&s1, 0);

                match assign {
                    "=" => self.reassign(name, &s2),
                    "+=" => self.plus_equal(name, &s2),
                    "-=" => self.minus_equal(name, &s2),
                    "*=" => self.times_equal(name, &s2),
                    "/=" => self.divide_equal(name, &s2),
                    "%=" => self.mod_equal(name, &s2),
                    "?=" => self.rand_equal(name, &s2),
                    "$=" => self.concatenate(name, &s2),
                    _ => self.assign_error(),
                }
                break;
            }

            self.assign_error();
            break;
        }
    }
}
