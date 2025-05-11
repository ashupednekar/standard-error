use crate::StandardError;

use askama::Template;

pub trait Render{
    fn render(&mut self, t: Template) -> Self;
}

impl Render for StandardError{
    fn render(&mut self, t: Template) -> Self {
        self.template = Some(t);
    }
}
