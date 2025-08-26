CREATE TABLE State (
    id SERIAL PRIMARY KEY,
    date timestamp DEFAULT NOW(),
    temperature float,
    health float,
    production_progress float,
    humidity float,
    fertility float,
    rentability float,
    pack_id varchar(200),
    FOREIGN KEY (pack_id) REFERENCES Sensor_pack(id)
);

