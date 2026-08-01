use crate::utils::cli;

pub struct Stream<'a> {
    print: Box<dyn FnMut(&str) + 'a>,
    input: Box<dyn FnMut(&str) -> String + 'a>,
}

impl<'a> Stream<'a> {
    pub fn new(
        print: impl FnMut(&str) + 'a,
        input: impl FnMut(&str) -> String + 'a,
    ) -> Self {
        Self {
            print: Box::new(print),
            input: Box::new(input),
        }
    }
    
    pub fn console() -> Self {
        Self {
            print: Box::new(|text| cli::print!("{}", text)),
            input: Box::new(|text| cli::input!("{}", text))
        }
    }

    pub fn black_hole() -> Self {
        Self {
            print: Box::new(|_| {}),
            input: Box::new(|_| { String::new() })
        }
    }

    pub fn print(&mut self, text: &str) {
        (self.print)(text);
    }

    pub fn input(&mut self, text: &str) -> String {
        (self.input)(text)
    }
}