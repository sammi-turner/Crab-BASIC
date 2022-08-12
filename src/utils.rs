use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use std::io::Read;
use std::io::Write;

// Prompts the user for input and returns it as a string.
pub fn input(prompt: &str) -> String {
    print!("\n{} > ", prompt);
    std::io::stdout().flush().unwrap();
    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer).unwrap();
    answer.pop();
    return answer;
}

// Clears the terminal window on SOME terminal emulators.
pub fn cls() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

// Seeds the pseudo-random number generator with unix time.
pub fn seed() {
    let d = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap();
    let _rng = StdRng::seed_from_u64(d.as_secs());
}

// Generates a pseudo-random number between x and y.
pub fn pseudo(x: i32, y: i32) -> i32 {
    return rand::thread_rng().gen_range(x..y + 1);
}

// Returns the nth char (zero indexed) from a slice.
pub fn nth_char(x: &str, n: usize) -> char {
    return x.chars().nth(n).unwrap();
}

// Checks if a slice consists only of digits.
pub fn is_digits(x: &str) -> bool {
    return x.chars().all(char::is_numeric);
}

// Checks if a slice represents a positive integer.
pub fn is_pos_int(x: &str) -> bool {
    if !is_digits(x) {
        return false;
    }
    if nth_char(x, 0) == '0' {
        return false;
    }
    return true;
}

// Checks if a slice represents a negative integer.
pub fn is_neg_int(x: &str) -> bool {
    if nth_char(x, 0) != '-' {
        return false;
    }
    if nth_char(x, 1) == '0' {
        return false;
    }
    if !nth_char(x, 1).is_ascii_digit() {
        return false;
    }

    let mut y = x.to_string();
    let _ = &mut y.remove(0);
    let _ = &mut y.remove(0);

    let z = y.as_str();
    if !is_digits(z) {
        return false;
    }
    return true;
}

// Checks if a slice represents an integer.
pub fn is_int(x: &str) -> bool {
    return is_pos_int(x) || x == "0" || is_neg_int(x);
}

// Converts a slice to an i32 integer.
pub fn to_int(x: &str) -> i32 {
    return x.parse::<i32>().unwrap();
}

// Counts the number of words in a slice.
pub fn word_count(x: &str) -> usize {
    if x == "" {
        return 0;
    }
    let mut count: usize = 0;
    let word_vec = x.split(" ");
    for _ in word_vec {
        count += 1;
    }
    return count;
}

// Counts the number of lines in a slice.
pub fn line_count(x: &str) -> usize {
    if x == "" {
        return 0;
    }
    let mut count: usize = 0;
    let word_vec = x.split("\n");
    for _ in word_vec {
        count += 1;
    }
    return count;
}

// Returns the nth word (zero indexed) from a slice.
pub fn nth_word(x: &str, y: usize) -> String {
    if x == "" || y >= word_count(x) {
        return "".to_string();
    }
    let word_vec = x.split(" ");
    let mut result = "";
    let mut count: usize = 0;
    for r in word_vec {
        if y == count {
            result = r;
        }
        count += 1;
    }
    result.to_string()
}

// Returns the nth line (zero indexed) from a slice.
pub fn nth_line(x: &str, y: usize) -> String {
    if x == "" || y >= line_count(x) {
        return "".to_string();
    }
    let line_vec = x.split("\n");
    let mut result = "";
    let mut count: usize = 0;
    for r in line_vec {
        if y == count {
            result = r;
        }
        count += 1;
    }
    result.to_string()
}

// Removes the nth word (zero indexed) from a slice and returns it as a string.
pub fn remove_nth_word(x: &str, y: usize) -> String {
    if x == "" || y >= word_count(x) {
        return x.to_string();
    }

    let mut result = String::new();
    let word_vec = x.split(" ");
    let mut count: usize = 0;

    for r in word_vec {
        if y != count {
            result.push_str(&r);
            result.push(' ');
        }
        count += 1;
    }

    let _ = result.pop();
    result
}

// Returns true if the file path exists.
pub fn file_exists(path: &str) -> bool {
    std::fs::metadata(path).is_ok()
}

// Reads from a file.
pub fn read_from_file(path: &str) -> String {
    let mut f = std::fs::File::open(path).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    buffer
}
