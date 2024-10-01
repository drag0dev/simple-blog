CREATE TABLE BlogPost (
     id SERIAL PRIMARY KEY,
     text VARCHAR(2000) NOT NULL,
     username VARCHAR(256) NOT NULL,
     dateOfPublication Date NOT NULL,
     avatar VARCHAR(512),
     image VARCHAR(512)
);
