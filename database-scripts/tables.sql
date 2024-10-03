CREATE TABLE BlogPost (
     id SERIAL PRIMARY KEY,
     text VARCHAR(2000) NOT NULL,
     username VARCHAR(128) NOT NULL,
     dateOfPublication Date NOT NULL
);

CREATE TABLE Image (
     id SERIAL PRIMARY KEY,
     image SMALLINT NOT NULL,
     path VARCHAR(512) NOT NULL,
     blogpostId INT REFERENCES BlogPost (id) NOT NULL
);
