use crate::exam::{Exam, Question};
use std::io::BufRead;
use log::{debug, trace};
use crate::exam::Answer::{Wrong, Correct};

#[derive(Debug, PartialEq)]
enum ParseState {
    None,
    QuestionHeader,
    Question,
    AnswerHeader,
    Answer,
}

// TODO: refactor
pub fn parse_exam<T: BufRead>(input: T) -> Result<Exam, &'static str> {
    const QUESTION_HEADER: &str = "[question]";
    const ANSWER_HEADER: &str = "[ans]";
    const CORRECT_HEADER: &str = "[ans*]";

    let mut state = ParseState::None;
    let mut questions = Vec::<Question>::new();
    let mut tmp = Question::new();
    let mut buffer = String::new();
    let mut is_correct = ANSWER_HEADER.to_string();
    let mut had_correct = false;

    let lines = input.lines()
        .map(|l| l.unwrap_or_else(|_| "\n".to_string()))
        .collect::<Vec<_>>();

    for line in lines {
        trace!("Current state: \"{:?}\"", state);
        trace!("Current line: \"{}\"", line);

        if line == CORRECT_HEADER {
            had_correct = true;
        }

        match state {
            ParseState::None => {
                if line == QUESTION_HEADER {
                    state = ParseState::QuestionHeader;
                } else {
                    return Err("No question in the first line");
                }
            }
            ParseState::QuestionHeader => {
                if line == QUESTION_HEADER || line == ANSWER_HEADER || line == CORRECT_HEADER {
                    return Err("No question content");
                }

                trace!("Adding \"{}\" to the header", line);
                buffer += format!("{}\n", line).as_str();
                state = ParseState::Question;
            }
            ParseState::Question => {
                if line == QUESTION_HEADER {
                    return Err("No defined answers");
                } else if line == ANSWER_HEADER || line == CORRECT_HEADER {
                    tmp.description = buffer.clone().trim().to_string();
                    buffer.clear();
                    trace!("Ready question:\n{}", tmp.description);
                    is_correct = line;
                    state = ParseState::AnswerHeader;
                } else {
                    trace!("Adding \"{}\" to the header", line);
                    buffer += format!("{}\n", line).as_str();
                    state = ParseState::Question;
                }
            }
            ParseState::AnswerHeader => {
                if line == QUESTION_HEADER || line == ANSWER_HEADER || line == CORRECT_HEADER {
                    return Err("No answer content");
                }
                trace!("Adding \"{}\" to the answer header", line);
                buffer += format!("{}\n", line).as_str();
                state = ParseState::Answer;
            }
            ParseState::Answer => {
                if line == QUESTION_HEADER {
                    if !had_correct {
                        return Err("Every question must contain at least one correct answer");
                    }

                    tmp.add_answer(Correct(buffer.clone().trim().to_string()));
                    buffer.clear();
                    debug!("Dumping question:\n{}", tmp);
                    questions.push(tmp);
                    tmp = Question::new();
                    had_correct = false;
                    trace!("had_correct = {}", had_correct);
                    state = ParseState::QuestionHeader;
                } else if line == ANSWER_HEADER || line == CORRECT_HEADER {
                    trace!("Dumping answer:\n{}", buffer);
                    tmp.add_answer(
                        if is_correct == CORRECT_HEADER {
                            Correct(buffer.trim().to_string())
                        } else {
                            Wrong(buffer.trim().to_string())
                        });
                    buffer.clear();
                    is_correct = line;
                    state = ParseState::AnswerHeader;
                } else {
                    trace!("Adding \"{}\" to the answer header", line);
                    buffer += format!("{}\n", line).as_str();
                    state = ParseState::Answer;
                }
            }
        }
    }

    if state != ParseState::Answer {
        return Err("Exam file parsing error");
    }

    trace!("Dumping answer:\n{}", buffer);
    tmp.add_answer(
        if is_correct == CORRECT_HEADER {
            Correct(buffer.trim().to_string())
        } else {
            Wrong(buffer.trim().to_string())
        });

    debug!("Dumping question:\n{}", tmp);
    questions.push(tmp);

    if !had_correct {
        return Err("Every question must contain at least one correct answer");
    }

    debug!("Finished parsing exam file");
    Ok(Exam { questions })
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use crate::parser::parse_exam;
    use crate::exam::{Exam, Question};
    use crate::exam::Answer::{Wrong, Correct};

    #[test]
    fn correct_exam_ok() {
        let exam_string = "[question]\n\
            Question text\n\
            \n\
            fork()\n\
            [ans]\n\
            Wrong answer\n\
            [ans*]\n\
            Correct answer\n\
            \n\
            [question]\n\
            Second question text\n\
            [ans*]\n\
            Good\n\
            [ans]\n\
            Bad\n\
            [ans]\n\
            Multiline\n\
            Also bad\n";

        let expected = Exam {
            questions: vec![Question {
                description: "Question text\n\nfork()".to_string(),
                answers: vec![
                    Wrong("Wrong answer".to_string()),
                    Correct("Correct answer".to_string())
                ],
            }, Question {
                description: "Second question text".to_string(),
                answers: vec![
                    Correct("Good".to_string()),
                    Wrong("Bad".to_string()),
                    Wrong("Multiline\nAlso bad".to_string())
                ],
            }],
        };

        let input = BufReader::new(exam_string.as_bytes());
        let actual = parse_exam(input);

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn no_answers_fails() {
        let exam_string = "[question]\n\
            Question text\n\
            [question]\n\
            Text2\n";

        let input = BufReader::new(exam_string.as_bytes());
        let actual = parse_exam(input);

        assert!(!actual.is_ok());
    }

    #[test]
    fn only_answers_fails() {
        let exam_string = "\n\
            Question text\n\
            \n\
            fork()\n\
            [ans]\n\
            Wrong answer\n\
            [ans*]\n\
            Correct answer\n\
            \n\
            Second question text\n\
            [ans*]\n\
            Good\n\
            [ans]\n\
            Bad\n\
            [ans]\n\
            Multiline\n\
            Also bad\n";

        let input = BufReader::new(exam_string.as_bytes());
        let actual = parse_exam(input);

        assert!(!actual.is_ok());
    }

    #[test]
    fn no_answer_content_fails() {
        let exam_string = "[question]\n\
            Question text\n\
            \n\
            fork()\n\
            [ans]\n\
            [ans*]\n\
            \n\
            [question]\n\
            Second question text\n\
            [ans*]\n\
            \n\n\
            [ans]\n\
            [ans]\n\n";

        let input = BufReader::new(exam_string.as_bytes());
        let actual = parse_exam(input);

        assert!(!actual.is_ok());
    }

    #[test]
    fn garbage_fails() {
        let exam_string = "wsdasdasdasda\nasdasdasdasd     asdas\n\n\n\n";

        let input = BufReader::new(exam_string.as_bytes());
        let actual = parse_exam(input);

        assert!(!actual.is_ok());
    }

    #[test]
    fn no_correct_answer_fails() {
        let exam_string = "[question]\n\
            Question text\n\
            \n\
            fork()\n\
            [ans]\n\
            Wrong answer\n\
            [ans]\n\
            Correct answer\n\
            \n\
            [question]\n\
            Second question text\n\
            [ans]\n\
            Good\n\
            [ans]\n\
            Bad\n\
            [ans*]\n\
            Multiline\n\
            Also bad\n";

        let input = BufReader::new(exam_string.as_bytes());
        let actual = parse_exam(input);

        assert!(!actual.is_ok());
    }

    #[test]
    fn empty_fails() {
        let exam_string = "";

        let input = BufReader::new(exam_string.as_bytes());
        let actual = parse_exam(input);

        assert!(!actual.is_ok());
    }
}
