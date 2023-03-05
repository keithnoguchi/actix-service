use actix_web::{web, HttpResponse};
use chrono::Utc;

use crate::model::Course;
use crate::state::State;

pub async fn new_course(course: web::Json<Course>, state: web::Data<State>) -> HttpResponse {
    let mut courses = state.courses.write().unwrap();
    let course_count = courses
        .iter()
        .filter(|c| c.tutor_id == course.tutor_id)
        .count() as u32;

    let new_course = Course {
        tutor_id: course.tutor_id,
        course_name: course.course_name.clone(),
        course_id: Some(course_count + 1),
        posted_time: Some(Utc::now().naive_utc()),
    };

    courses.push(new_course);
    HttpResponse::Ok().json("Added course")
}

pub async fn get_courses(state: web::Data<State>, path: web::Path<u32>) -> HttpResponse {
    let tutor_id = path.into_inner();
    let courses: Vec<Course> = state
        .courses
        .read()
        .unwrap()
        .iter()
        .filter(|c| c.tutor_id == tutor_id)
        .cloned()
        .collect();

    HttpResponse::Ok().json(courses)
}

pub async fn get_course(state: web::Data<State>, path: web::Path<(u32, u32)>) -> HttpResponse {
    let (tutor_id, course_id) = path.into_inner();
    let course = state
        .courses
        .read()
        .unwrap()
        .iter()
        .find(|c| c.tutor_id == tutor_id && c.course_id == Some(course_id))
        .cloned();

    if let Some(course) = course {
        HttpResponse::Ok().json(course)
    } else {
        HttpResponse::Ok().json("Course not found")
    }
}

pub(crate) async fn health(state: web::Data<State>) -> HttpResponse {
    let msg = &state.message;
    let resp = {
        let mut count = state.count.lock().unwrap();
        let course_count = state.courses.read().unwrap().len();
        let resp = format!("{msg} {count} times with {course_count} courses");
        *count += 1;
        resp
    };
    HttpResponse::Ok().json(resp)
}
