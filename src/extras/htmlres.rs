#[cfg(feature = "askama")]
use crate::StandardError;

#[cfg(feature = "askama")]
pub trait HtmlRes {
    fn template(&mut self, t: String) -> Self;
}

#[cfg(feature = "askama")]
impl HtmlRes for StandardError {
    fn template(&mut self, t: String) -> Self {
        self.html = Some(t);
        self.clone()
    }
}
