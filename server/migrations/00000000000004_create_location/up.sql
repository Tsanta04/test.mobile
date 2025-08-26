CREATE TABLE Location (
    id SERIAL PRIMARY KEY,
    longitude float NOT NULL,
    latitude float NOT NULL,
    city varchar(200) NOT NULL,
    country varchar(200) NOT NULL
);

