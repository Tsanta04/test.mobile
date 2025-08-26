CREATE TABLE Person (
    id SERIAL PRIMARY KEY,
    name varchar(200) NOT NULL,
    firstname varchar(200),
    date_birth timestamp,
    location_birth varchar(200),
    number varchar(10),
    cin varchar(15),
    user_id int,
    FOREIGN KEY (user_id) REFERENCES Users(user_id)
);

