-- Create custom types
CREATE TYPE AlertType AS ENUM ('Health', 'Production', 'Rentability', 'Humidity', 'Fertility', 'Other');
CREATE TYPE LevelType AS ENUM ('Low', 'Medium', 'High', 'Urgent');

-- Create tables
CREATE TABLE Users (
    user_id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    hashed_password TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('seller', 'buyer', 'supplier'))
);

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

CREATE TABLE Culture_type (
    id SERIAL PRIMARY KEY,
    type varchar(200)
);

CREATE TABLE Location (
    id SERIAL PRIMARY KEY,
    longitude float NOT NULL,
    latitude float NOT NULL,
    city varchar(200) NOT NULL,
    country varchar(200) NOT NULL
);

CREATE TABLE Sensor_type (
    id SERIAL PRIMARY KEY,
    type varchar(200)
);

CREATE TABLE Sensor (
    id SERIAL PRIMARY KEY,
    description varchar(200),
    issue_date timestamp DEFAULT NOW(),
    sensor_type int,
    FOREIGN KEY (sensor_type) REFERENCES Sensor_type(id)
);

CREATE TABLE Sensor_pack (
    id varchar(200) PRIMARY KEY,
    description varchar(200)
);

CREATE TABLE Pack (
    id SERIAL PRIMARY KEY,    
    pack_id varchar(200),
    sensor_id int,
    FOREIGN KEY (pack_id) REFERENCES Sensor_pack(id),
    FOREIGN KEY (sensor_id) REFERENCES Sensor(id)
);

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

CREATE TABLE Discussion (
    id SERIAL PRIMARY KEY,
    name varchar(200) NOT NULL,
    initialised_at timestamp DEFAULT NOW()
);

CREATE TABLE Message (
    id SERIAL PRIMARY KEY,
    sender_id int,
    content varchar(200),
    discussion_id int,
    FOREIGN KEY (sender_id) REFERENCES Users(user_id),
    FOREIGN KEY (discussion_id) REFERENCES Discussion(id)
);

CREATE TABLE Participant (
    id SERIAL PRIMARY KEY,
    discussion_id int,
    participant_id int,
    FOREIGN KEY (participant_id) REFERENCES Users(user_id),
    FOREIGN KEY (discussion_id) REFERENCES Discussion(id)
);

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

CREATE TABLE Alert (
    id SERIAL PRIMARY KEY,
    date timestamp DEFAULT NOW(),
    title varchar(200) NOT NULL,
    description varchar(200) NOT NULL,
    type AlertType not null,
    level LevelType DEFAULT 'Low',
    recommandation varchar(200),
    isSeen boolean DEFAULT false,
    state_id int,
    FOREIGN KEY (state_id) REFERENCES State(id)
);

CREATE TABLE Planning (
    id SERIAL PRIMARY KEY,
    date timestamp DEFAULT NOW(),
    title varchar(200) NOT NULL,
    description varchar(200) NOT NULL,
    start_date timestamp DEFAULT NOW(),
    end_date timestamp DEFAULT NOW(),
    ground int,
    FOREIGN KEY (ground) REFERENCES Ground(id)
);

