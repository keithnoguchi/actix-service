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
