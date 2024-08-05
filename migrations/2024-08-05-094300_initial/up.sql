-- Your SQL goes here
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
    title VARCHAR (50)
);

CREATE TABLE photos (
    id SERIAL PRIMARY KEY,
    path TEXT NOT NULL UNIQUE,
    title VARCHAR (50),
    user_id INT NOT NULL,
    CONSTRAINT fk_photos_users
      FOREIGN KEY(user_id) 
        REFERENCES users(id),
    album_id INT,
    CONSTRAINT fk_photos_albums
      FOREIGN KEY(album_id) 
        REFERENCES albums(id)
);