use actix_web::web;
use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Course {
    pub(crate) course_id: u32,
    pub(crate) tutor_id: u32,
    pub(crate) course_name: String,
    pub(crate) posted_time: Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        Self {
            course_id: course.course_id,
            tutor_id: course.tutor_id,
            course_name: course.course_name.clone(),
            posted_time: course.posted_time,
        }
    }
}
