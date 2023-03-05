use crate::model::Course;
use eyre::Result;
use sqlx::postgres::PgPool;

pub(crate) async fn add_course(pool: &PgPool, course: Course) -> Result<Course> {
    let course_row = sqlx::query!(
        r#"
            INSERT INTO course_c4
            (course_id, tutor_id, course_name)
            VALUES($1, $2, $3)
            RETURNING course_id, tutor_id, course_name, posted_time
        "#,
        course.course_id as i32,
        course.tutor_id as i32,
        course.course_name,
    )
    .fetch_one(pool)
    .await?;

    Ok(Course {
        course_id: course_row.course_id as u32,
        tutor_id: course_row.tutor_id as u32,
        course_name: course_row.course_name,
        posted_time: course_row.posted_time,
    })
}

pub(crate) async fn get_courses(pool: &PgPool, tutor_id: u32) -> Result<Vec<Course>> {
    let course_rows = sqlx::query!(
        r#"
            SELECT tutor_id, course_id, course_name, posted_time FROM course_c4
            WHERE tutor_id = $1
        "#,
        tutor_id as i32,
    )
    .fetch_all(pool)
    .await?;

    Ok(course_rows
        .into_iter()
        .map(|row| Course {
            course_id: row.course_id as u32,
            tutor_id: row.tutor_id as u32,
            course_name: row.course_name,
            posted_time: row.posted_time,
        })
        .collect())
}

pub(crate) async fn get_course(pool: &PgPool, tutor_id: u32, course_id: u32) -> Result<Course> {
    let course_row = sqlx::query!(
        r#"
            SELECT tutor_id, course_id, course_name, posted_time FROM course_c4
            WHERE tutor_id = $1 AND course_id = $2
        "#,
        tutor_id as i32,
        course_id as i32,
    )
    .fetch_one(pool)
    .await?;

    Ok(Course {
        course_id: course_row.course_id as u32,
        tutor_id: course_row.tutor_id as u32,
        course_name: course_row.course_name,
        posted_time: course_row.posted_time,
    })
}
