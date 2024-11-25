-- Your SQL goes here
CREATE EXTENSION vector;


CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR (50) UNIQUE NOT NULL,
    email VARCHAR (50) UNIQUE NOT NULL,
    password VARCHAR (256) NOT NULL,
    avatar TEXT,
    is_admin BOOLEAN NOT NULL
);

CREATE TABLE albums (
    id SERIAL PRIMARY KEY,
    title VARCHAR (50) NOT NULL
);

CREATE TABLE photos (
    id SERIAL PRIMARY KEY,
    path TEXT,
    title VARCHAR (50),
    embedding VECTOR(512),
    user_id INT NOT NULL,
    CONSTRAINT fk_photos_users
      FOREIGN KEY(user_id)
        REFERENCES users(id),
    album_id INT,
    CONSTRAINT fk_photos_albums
      FOREIGN KEY(album_id)
        REFERENCES albums(id)
);


CREATE TABLE persons (
    id SERIAL PRIMARY KEY,
    title VARCHAR (50) NOT NULL DEFAULT 'Unknown',
    avatar TEXT NOT NULL
);


CREATE TABLE faces (
    id SERIAL PRIMARY KEY,
    person_id INT,
    photo_id INT NOT NULL,
    embedding VECTOR(512),
    path TEXT,
    bbox INT[4],
    CONSTRAINT fk_faces_persons
      FOREIGN KEY(person_id)
        REFERENCES persons(id),
    CONSTRAINT fk_faces_photos
      FOREIGN KEY(photo_id)
        REFERENCES photos(id)
);
