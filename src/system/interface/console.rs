use crate::system::interface::Input::{Invalid, Message, Quit};
use crate::system::interface::{ExamIO, Input};
use crate::system::exam::Question;
use getch::Getch;


pub struct ConsoleIO {
    get: Getch,
}

impl ExamIO for ConsoleIO {
    fn new() -> Self {
        ConsoleIO { get: Getch::new() }
    }

    fn get_input(&self) -> Input {
        let input = self.get.getch().unwrap();

        if input as char == 'q' {
            return Quit;
        }

        match (input as char).to_digit(10) {
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
