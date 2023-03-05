use std::sync::atomic::AtomicUsize;

use sqlx::postgres::PgPool;

pub struct State {
    pub health_check_string: String,
    pub visit_count: AtomicUsize,
    pub db: PgPool,
}
