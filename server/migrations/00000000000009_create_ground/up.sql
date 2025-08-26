CREATE TABLE Ground (
    id SERIAL PRIMARY KEY,
    name varchar(200) NOT NULL,
    description varchar(200) NOT NULL,
    folder varchar(200),
    culture_type int,
    location int,
    user_id int,
    pack varchar(200),
    FOREIGN KEY (location) REFERENCES Location(id),
    FOREIGN KEY (culture_type) REFERENCES Culture_type(id),
    FOREIGN KEY (user_id) REFERENCES Users(user_id),
    FOREIGN KEY (pack) REFERENCES Sensor_pack(id)    
);

