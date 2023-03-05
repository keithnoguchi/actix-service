use std::sync::{Mutex, RwLock};

use crate::model::Course;

#[derive(Default)]
pub struct State {
    pub(crate) message: String,
    pub(crate) count: Mutex<u32>,
    pub(crate) courses: RwLock<Vec<Course>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            message: "I'm good, thank you".into(),
            ..Self::default()
        }
    }
}
