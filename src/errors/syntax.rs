use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use super::{Bt2Error, Bt2Result};

#[derive(Debug)]
pub struct SyntaxError {
    message: String,
    sub_messages: Vec<String>,
}

impl SyntaxError {
    pub fn message(&self) -> &str { &self.message }
    pub fn sub_messages(&self) -> &[String] { &self.sub_messages }

    pub fn new(msg: &str) -> Self {
        Self {
            message: String::from(msg),
            sub_messages: vec![],
        }
    }

    pub fn with_sub_message(mut self, sub_msg: &str) -> Self {
        self.sub_messages.push(String::from(sub_msg));
        self
    }

    pub fn with_line(self, lin: usize) -> Self {
        let info_string = format!("At line {}", lin);
        self.with_sub_message(&info_string)
    }

    pub fn with_line_and_token(self, lin: usize, tok: usize) -> Self {
        let info_string = format!("At line {}, token {}", lin, tok);
        self.with_sub_message(&info_string)
    }

    pub fn with_line_and_column(self, lin: usize, col: usize) -> Self {
        let info_string = format!("At line {}, column {}", lin, col);
        self.with_sub_message(&info_string)
    }

    pub fn with_presentation<T: Display>(self, title: &str, value: &T) -> Self {
        let info_string = format!("{}: {}", title, value);
        self.with_sub_message(&info_string)
    }

    pub fn with_code_snippet(self, snippet: &str) -> Self {
        let info_string = format!(">> {}", snippet);
        self.with_sub_message(&info_string)
    }

    pub fn raise<T>(self) -> Bt2Result<T> {
        Err(Bt2Error::Syntax(Self {
            message: self.message,
            sub_messages: self.sub_messages,
        }))
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "--- SYNTAX ERROR ---\n{}", self.message)?;
        for sub_message in &self.sub_messages {
            write!(f, "\n{}", sub_message)?;
        }
        Ok(())
    }
}

impl Error for SyntaxError {}