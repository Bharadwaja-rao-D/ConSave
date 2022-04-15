-- Your SQL goes here

create table user_info(
	id serial primary key,
	name text not null,
	password text not null
);

create table post(
	post_id serial primary key,
	title text not null,
	content text not null,
	user_id int references user_info(id) not null
);
