use std::env;
use std::error::Error;

use dotenvy::dotenv;
use sqlx::postgres::PgPool;

#[derive(Debug)]
pub struct Course {
    pub course_id: u32,
    pub tutor_id: u32,
    pub course_name: String,
    pub posted_time: Option<chrono::NaiveDateTime>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;
    let database_url = env::var("DATABASE_URL")?;
    let db_pool = PgPool::connect(&database_url).await?;
    let course_rows = sqlx::query!(
        r#"
            select course_id, tutor_id, course_name, posted_time from course_c4
            where tutor_id = $1
        "#,
        1,
    )
    .fetch_all(&db_pool)
    .await?;
    let mut course_list = vec![];
    for row in course_rows {
        course_list.push(Course {
            course_id: row.course_id as u32,
            tutor_id: row.tutor_id as u32,
            course_name: row.course_name,
            posted_time: row.posted_time,
        });
    }
    println!("{course_list:#?}");
    Ok(())
}
