-- Your SQL goes here
CREATE TABLE persons (
    id SERIAL PRIMARY KEY,
    title VARCHAR (50) NOT NULL DEFAULT 'Unknown',
    avatar TEXT NOT NULL UNIQUE
);


CREATE TABLE faces (
    person_id INT NOT NULL,
    photo_id INT NOT NULL,
    embedding VECTOR(512),
    bbox INT[4],
    CONSTRAINT fk_faces_persons
      FOREIGN KEY(person_id) 
        REFERENCES persons(id),
    CONSTRAINT fk_faces_photos
      FOREIGN KEY(photo_id)
        REFERENCES photos(id),
    PRIMARY KEY (person_id, photo_id)
);