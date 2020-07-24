pub mod console;

use crate::system::exam::Question;

pub enum Input {
    Message(usize),
    Invalid,
    Quit,
}

pub trait ExamIO {
    fn new() -> Self;
    fn get_input(&self) -> Input;
    fn show_question(&self, q: &Question);
    fn show_result(&self, pick: bool);
}
