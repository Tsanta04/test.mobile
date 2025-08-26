CREATE TABLE Sensor (
    id SERIAL PRIMARY KEY,
    description varchar(200),
    issue_date timestamp DEFAULT NOW(),
    sensor_type int,
    FOREIGN KEY (sensor_type) REFERENCES Sensor_type(id)
);

