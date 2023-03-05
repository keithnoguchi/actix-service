use actix_web::{http, web};
use tutor_nodb::{handler, model, state};

#[tokio::test]
async fn new_course_handler() {
    let course = web::Json(model::Course {
        tutor_id: 1,
        course_name: "Hello, this is the first course".to_string(),
        course_id: None,
        posted_time: None,
    });
    let state = web::Data::new(state::State::default());

    let resp = handler::new_course(course, state).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
}

#[tokio::test]
async fn get_courses_handler() {
    let state = web::Data::new(state::State::default());
    let path = web::Path::from(1);

    let resp = handler::get_courses(state, path).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
}

#[tokio::test]
async fn get_course_handler() {
    let state = web::Data::new(state::State::default());
    let path = web::Path::from((1, 1));

    let resp = handler::get_course(state, path).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
}
