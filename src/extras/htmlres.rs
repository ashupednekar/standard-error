use crate::StandardError;

pub trait HtmlRes{
    fn render(&mut self, t: String) -> Self;
}

impl HtmlRes for StandardError{
    fn render(&mut self, t: String) -> Self {
        self.html = Some(t);
        self.clone()
    }
}
