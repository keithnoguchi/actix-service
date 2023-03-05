use std::sync::atomic::AtomicUsize;

use sqlx::postgres::PgPool;

#[derive(Debug)]
pub struct State {
    pub health_check_string: String,
    pub visit_count: AtomicUsize,
    pub db: PgPool,
}

impl State {
    pub fn new(db: PgPool) -> Self {
        Self {
            health_check_string: "I'm good.".to_string(),
            visit_count: AtomicUsize::new(0),
            db,
        }
    }
}
