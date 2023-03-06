use std::sync::atomic::AtomicUsize;

#[derive(Debug, Default)]
pub struct State {
    pub(crate) msg: String,
    pub(crate) counter: AtomicUsize,
}

impl State {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
            ..Default::default()
        }
    }
}
