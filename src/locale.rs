use std::cell::RefCell;
use std::env;

thread_local! {
    static LOCALE: RefCell<String> = RefCell::new(env::var("DEFAULT_LOCALE").unwrap_or("en_US".to_string()));
}

pub fn set_current_locale(locale: &str) {
    LOCALE.with(|s| {
        *s.borrow_mut() = locale.to_string();
    })
}

pub fn get_current_locale() -> String {
    LOCALE.with(|s| s.borrow().clone())
}


