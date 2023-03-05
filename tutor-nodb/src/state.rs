use std::sync::Mutex;

pub struct State {
    pub message: String,
    pub count: Mutex<u32>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            message: "I'm good, thank you".into(),
            count: Mutex::new(0),
        }
    }
}
