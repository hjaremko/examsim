use rand::Rng;
use std::fmt;

pub enum Answer {
    Ok(String),
    Wrong(String),
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Answer::Ok(ref ans) => write!(f, "{}", ans),
            Answer::Wrong(ref ans) => write!(f, "{}", ans),
        }
    }
}

pub struct Question {
    pub description: String,
    pub answers: Vec<Answer>,
}

impl Question {
    pub fn is_correct(&self, input: usize) -> Option<bool> {
        if input >= self.answers.len() {
            return None;
        }

        match self.answers[input] {
            Answer::Ok(_) => Some(true),
            Answer::Wrong(_) => Some(false),
        }
    }
}

impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "-- {} --", self.description).unwrap();
        for (i, ans) in self.answers.iter().enumerate() {
            writeln!(f, " {}) {}", i, ans).unwrap();
        }

        Ok(())
    }
}

pub struct Exam {
    questions: Vec<Question>,
}

impl Exam {
    pub fn select(&self) -> &Question {
        let mut rng = rand::thread_rng();

        let question_number = rng.gen_range(0, self.questions.len());
        &self.questions[question_number]
    }

    pub fn new() -> Exam {
        let v = vec![
            Answer::Ok(String::from("Ok")),
            Answer::Wrong(String::from("Wrong")),
        ];
        let v2 = vec![
            Answer::Wrong(String::from("Wrong")),
            Answer::Ok(String::from("Ok")),
        ];

        let q = Question {
            description: String::from("Question 1"),
            answers: v,
        };

        let q2 = Question {
            description: String::from("Question 2"),
            answers: v2,
        };

        Exam {
            questions: vec![q, q2],
        }
    }
}
