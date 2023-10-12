---- Base app schema

-- Authors
CREATE TABLE "authors" (
	id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,
	name varchar(128) NOT NULL UNIQUE,
	email varchar(128) NOT NULL UNIQUE,
	password varchar(256) NOT NULL
);

-- Posts
CREATE TABLE posts (
	id BIGINT GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
	title varchar(256) NOT NULL,
	content varchar(512) NOT NULL,
	author_id INTEGER,
	FOREIGN KEY (author_id) REFERENCES authors(id) ON DELETE SET NULL
)