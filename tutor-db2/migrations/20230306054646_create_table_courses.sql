-- Add migration script here
DROP TABLE IF EXISTS course5;
CREATE TABLE course5 (
	course_id serial PRIMARY KEY,
	tutor_id INT NOT NULL,
	course_name TEXT NOT NULL,
	posted_time TIMESTAMP default now()
);
INSERT INTO course5
	(course_id, tutor_id, course_name, posted_time)
	VALUES(1, 1, 'First course', '2022-12-17 05:40:00');
INSERT INTO course5
	(course_id, tutor_id, course_name, posted_time)
	VALUES(2, 1, 'Second course', '2022-12-19 05:45:00');
