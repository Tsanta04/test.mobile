CREATE TABLE Pack (
    id SERIAL PRIMARY KEY,    
    pack_id varchar(200),
    sensor_id int,
    FOREIGN KEY (pack_id) REFERENCES Sensor_pack(id),
    FOREIGN KEY (sensor_id) REFERENCES Sensor(id)
);

