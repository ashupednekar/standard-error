use crate::StandardError;

pub trait HtmlRes {
    fn template(&mut self, t: String) -> Self;
}

impl HtmlRes for StandardError {
    fn template(&mut self, t: String) -> Self {
        self.html = Some(t);
        self.clone()
    }
}
