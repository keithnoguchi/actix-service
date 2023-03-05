drop table if exists course_c4;
create table course_c4 (
	course_id serial primary key, 
	tutor_id INT not null,
	course_name varchar(140) not null,
	posted_time TIMESTAMP default now()
);

insert into course_c4
	(course_id, tutor_id, course_name, posted_time)
	values(1, 1, 'First course', '2022-12-17 05:40:00');
insert into course_c4
	(course_id, tutor_id, course_name, posted_time)
	values(2, 1, 'Second course', '2022-12-18 05:45:00');
