use std::sync::atomic::AtomicUsize;

use sqlx::postgres::PgPool;

#[derive(Debug)]
pub struct State {
    pub(crate) msg: String,
    pub(crate) counter: AtomicUsize,
    pub(crate) db: PgPool,
}

impl State {
    pub fn new(msg: &str, db: PgPool) -> Self {
        Self {
            msg: msg.to_string(),
            counter: AtomicUsize::new(0),
            db,
        }
    }
}
