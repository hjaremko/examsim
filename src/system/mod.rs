mod exam;
mod interface;

use crate::system::exam::Exam;
use crate::system::interface::Input::Message;
use crate::system::interface::{ExamIO, Input};
use crate::system::interface::console::ConsoleIO;

pub struct System<IO> {
    io: IO,
    exam_engine: Exam,
}

impl<IO: ExamIO> System<IO> {
    pub fn new() -> System<IO> {
        System {
            io: IO::new(),
            exam_engine: Exam::new(),
        }
    }

    pub fn run(&self) {
        let mut q = self.exam_engine.select();
        self.io.show_question(&q);

        loop {
            match self.io.get_input() {
                Message(input) => {
                    if let Some(result) = q.is_correct(input) {
                        self.io.show_result(result);

                        q = self.exam_engine.select();
                        self.io.show_question(&q);
                    }
                }
                Input::Invalid => eprintln!("Invalid input"),
                Input::Quit => break,
            }
        }
    }
}

pub type ConsoleApp = System<ConsoleIO>;
