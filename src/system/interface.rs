use crate::exam::Question;
use crate::system::interface::Input::{Invalid, Message, Quit};
use getch::Getch;

#[derive(Debug, PartialEq)]
pub enum Input {
    Message(usize),
    Invalid,
    Quit,
}

pub trait ExamIO {
    fn new() -> Self;
    fn get_input(&self) -> Input;
    fn validate_input(input: char) -> Input;
    fn show_question(&self, q: &Question);
    fn show_result(&self, pick: bool);
}

pub struct ConsoleIO {
    get: Getch,
}

impl ExamIO for ConsoleIO {
    fn new() -> Self {
        ConsoleIO { get: Getch::new() }
    }

    fn get_input(&self) -> Input {
        Self::validate_input(self.get.getch().unwrap() as char)
    }

    fn validate_input(input: char) -> Input {
        if input == 'q' {
            return Quit;
        }

        match input.to_digit(10) {
            Some(i) => Message(i as usize),
            None => Invalid,
        }
    }

    fn show_question(&self, q: &Question) {
        white_ln!("> {}", q.description);

        for (i, ans) in q.answers.iter().enumerate() {
            println!(" {}) {}", i, ans);
        }
    }

    fn show_result(&self, pick: bool) {
        if pick {
            green_ln! {"Correct answer"}
        } else {
            red_ln! {"Wrong answer"}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::system::interface::Input::{Invalid, Message, Quit};
    use crate::system::interface::{ConsoleIO, ExamIO};

    #[test]
    fn get_input_correct() {
        assert_eq!(ConsoleIO::validate_input('0'), Message(0));
    }

    #[test]
    fn get_input_char() {
        assert_eq!(ConsoleIO::validate_input('h'), Invalid);
    }

    #[test]
    fn get_input_quit() {
        assert_eq!(ConsoleIO::validate_input('q'), Quit);
    }
}
