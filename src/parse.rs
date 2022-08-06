use crate::utils::*;
use crate::Runtime;

impl Runtime {
    pub fn parse_equal(&mut self, x: String, y: String) {
        if x != y {
            self.if_condition = false;
        }
    }

    pub fn parse_unequal(&mut self, x: String, y: String) {
        if x == y {
            self.if_condition = false;
        }
    }

    pub fn parse_gt(&mut self, x: String, y: String) {
        loop {
            if !is_int(&x) || !is_int(&y) {
                self.end_msg = format!(
                    "On line {}, the comparison is not defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            let vx = to_int(&x);
            let vy = to_int(&y);

            if vx <= vy {
                self.if_condition = false;
                break;
            }

            break;
        }
    }

    pub fn parse_lt(&mut self, x: String, y: String) {
        loop {
            if !is_int(&x) || !is_int(&y) {
                self.end_msg = format!(
                    "On line {}, the comparison is not defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            let vx = to_int(&x);
            let vy = to_int(&y);

            if vx >= vy {
                self.if_condition = false;
                break;
            }

            break;
        }
    }

    pub fn parse_gte(&mut self, x: String, y: String) {
        loop {
            if !is_int(&x) || !is_int(&y) {
                self.end_msg = format!(
                    "On line {}, the comparison is not defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            let vx = to_int(&x);
            let vy = to_int(&y);

            if vx < vy {
                self.if_condition = false;
                break;
            }

            break;
        }
    }

    pub fn parse_lte(&mut self, x: String, y: String) {
        loop {
            if !is_int(&x) || !is_int(&y) {
                self.end_msg = format!(
                    "On line {}, the comparison is not defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            let vx = to_int(&x);
            let vy = to_int(&y);

            if vx > vy {
                self.if_condition = false;
                break;
            }

            break;
        }
    }

    pub fn parse_and(&mut self, x: String, y: String) {
        loop {
            let xb = &x == "true" || &x == "false";
            let yb = &y == "true" || &x == "false";

            if !xb || !yb {
                self.end_msg = format!(
                    "On line {}, the comparison is not defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            if x != "true" || y != "true" {
                self.if_condition = false;
                break;
            }
        }
    }

    pub fn parse_or(&mut self, x: String, y: String) {
        loop {
            let xb = &x == "true" || &x == "false";
            let yb = &y == "true" || &x == "false";

            if !xb || !yb {
                self.end_msg = format!(
                    "On line {}, the comparison is not defined.",
                    self.current_line + 1
                );
                self.current_line = self.program.len();
                break;
            }

            if x == "false" && y == "false" {
                self.if_condition = false;
                break;
            }

            break;
        }
    }

    pub fn operator_error(&mut self) {
        self.end_msg = format!(
            "On line {}, the operator is invalid.",
            self.current_line + 1
        );
        self.current_line = self.program.len();
    }

    pub fn valid_name(&mut self, x: &str) -> bool {
        let first_char = nth_char(&x, 0);
        let lower = first_char.is_ascii_lowercase();
        let reserved = vec![
            "#", "if", "or", "and", "cls", "let", "end", "goto", "true", "false", "input", "print",
            "iprint",
        ];
        let is_reserved = reserved.contains(&x);
        return lower && !is_reserved;
    }

    pub fn reassign(&mut self, x: &str, y: &str) {
        loop {
            if self.if_condition == false {
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

    pub fn plus_equal(&mut self, x: &str, y: &str) {
        loop {
            if self.if_condition == false {
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

    pub fn minus_equal(&mut self, x: &str, y: &str) {
        loop {
            if self.if_condition == false {
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

    pub fn times_equal(&mut self, x: &str, y: &str) {
        loop {
            if self.if_condition == false {
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

    pub fn divide_equal(&mut self, x: &str, y: &str) {
        loop {
            if self.if_condition == false {
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

    pub fn mod_equal(&mut self, x: &str, y: &str) {
        loop {
            if self.if_condition == false {
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

    pub fn rand_equal(&mut self, x: &str, y: &str) {
        loop {
            if self.if_condition == false {
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

    pub fn concatenate(&mut self, x: &str, y: &str) {
        loop {
            if self.if_condition == false {
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

            let mut yv = y.to_string();
            if self.idents.contains_key(y) {
                yv = self.idents[y].clone();
            }

            let f = format!("{}{}", &self.idents[x], &yv);

            self.idents.remove(x);
            self.idents.insert(x.to_string(), f);
            break;
        }
    }

    pub fn assign_error(&mut self) {
        self.end_msg = format!("On line {}, that token is not defined.", self.current_line);
        self.current_line = self.program.len();
    }
}
