CREATE TABLE BlogPost (
     id SERIAL PRIMARY KEY,
     text VARCHAR(2000) NOT NULL,
     username VARCHAR(128) NOT NULL,
     dateOfPublication Date NOT NULL,
     avatar VARCHAR(128),
     postimage VARCHAR(128)
);
