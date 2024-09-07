use std::collections::HashMap;

use crate::StandardError;

pub trait Interpolate{
    fn interpolate_values(&mut self, values: HashMap<String, String>) -> Self;
    fn interpolate_err(&mut self, e: String) -> Self;
}

impl Interpolate for StandardError{
    fn interpolate_values(&mut self, values: HashMap<String, String>) -> Self {
        let mut new_message = self.message.clone(); // Clone the message to avoid mutating it directly
        for (k, v) in values.into_iter() {
            new_message = new_message.replace(&format!("[{}]", &k), &v); // Use new_message to accumulate changes
        }
        self.message = new_message;
        self.clone()
    }

    fn interpolate_err(&mut self, e: String) -> Self {
        let mut values: HashMap<String, String> = HashMap::new();
        values.insert("err".to_string(), e);
        self.interpolate_values(values)
    }

}
