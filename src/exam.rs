use rand::Rng;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Answer {
    Correct(String),
    Wrong(String),
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Answer::Correct(ref ans) => write!(f, "{}", ans),
            Answer::Wrong(ref ans) => write!(f, "{}", ans),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Question {
    pub description: String,
    pub answers: Vec<Answer>,
}

impl Question {
    pub fn new() -> Self {
        Question { description: "".to_string(), answers: vec![] }
    }

    pub fn add_answer(&mut self, answer: Answer) {
        self.answers.push(answer);
    }

    pub fn is_correct(&self, input: usize) -> Option<bool> {
        if input >= self.answers.len() {
            return None;
        }

        match self.answers[input] {
            Answer::Correct(_) => Some(true),
            Answer::Wrong(_) => Some(false),
        }
    }

    pub fn shuffle_answers(&self) {
        // unimplemented!();
    }
}

impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.description).unwrap();
        for (i, ans) in self.answers.iter().enumerate() {
            writeln!(f, " {}) {}", i, ans).unwrap();
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Exam {
    pub(crate) questions: Vec<Question>,
}

impl Exam {
    pub fn select(&self) -> &Question {
        let mut rng = rand::thread_rng();

        let question_number = rng.gen_range(0, self.questions.len());
        &self.questions[question_number]
    }
}
