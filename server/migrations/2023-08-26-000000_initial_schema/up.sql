-- Create Users table
CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    hashed_password TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('seller', 'buyer', 'supplier'))
);

-- Create Person table
CREATE TABLE person (
    id SERIAL PRIMARY KEY,
    name varchar(200) NOT NULL,
    firstname varchar(200),
    date_birth timestamp,
    location_birth varchar(200),
    number varchar(10),
    cin varchar(15),
    user_id int,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);

-- Create Culture_type table
CREATE TABLE culture_type (
    id SERIAL PRIMARY KEY,
    type varchar(200)
);

-- Create Location table
CREATE TABLE location (
    id SERIAL PRIMARY KEY,
    longitude float NOT NULL,
    latitude float NOT NULL,
    city varchar(200) NOT NULL,
    country varchar(200) NOT NULL
);

-- Create Sensor_type table
CREATE TABLE sensor_type (
    id SERIAL PRIMARY KEY,
    type varchar(200)
);

-- Create Sensor table
CREATE TABLE sensor (
    id SERIAL PRIMARY KEY,
    description varchar(200),
    issue_date timestamp DEFAULT NOW(),
    sensor_type int,
    FOREIGN KEY (sensor_type) REFERENCES sensor_type(id)
);

-- Create Sensor_pack table
CREATE TABLE sensor_pack (
    id varchar(200) PRIMARY KEY,
    description varchar(200)
);

-- Create Pack table
CREATE TABLE pack (
    id SERIAL PRIMARY KEY,    
    pack_id varchar(200),
    sensor_id int,
    FOREIGN KEY (pack_id) REFERENCES sensor_pack(id),
    FOREIGN KEY (sensor_id) REFERENCES sensor(id)
);

-- Create Ground table
CREATE TABLE ground (
    id SERIAL PRIMARY KEY,
    name varchar(200) NOT NULL,
    description varchar(200) NOT NULL,
    folder varchar(200),
    culture_type int,
    location int,
    user_id int,
    pack varchar(200),
    FOREIGN KEY (location) REFERENCES location(id),
    FOREIGN KEY (culture_type) REFERENCES culture_type(id),
    FOREIGN KEY (user_id) REFERENCES users(user_id),
    FOREIGN KEY (pack) REFERENCES sensor_pack(id)    
);

-- Create Discussion table
CREATE TABLE discussion (
    id SERIAL PRIMARY KEY,
    name varchar(200) NOT NULL,
    initialised_at timestamp DEFAULT NOW()
);

-- Create Message table
CREATE TABLE message (
    id SERIAL PRIMARY KEY,
    sender_id int,
    content varchar(200),
    discussion_id int,
    FOREIGN KEY (sender_id) REFERENCES users(user_id),
    FOREIGN KEY (discussion_id) REFERENCES discussion(id)
);

-- Create Participant table
CREATE TABLE participant (
    id SERIAL PRIMARY KEY,
    discussion_id int,
    participant_id int,
    FOREIGN KEY (participant_id) REFERENCES users(user_id),
    FOREIGN KEY (discussion_id) REFERENCES discussion(id)
);

-- Create State table
CREATE TABLE state (
    id SERIAL PRIMARY KEY,
    date timestamp DEFAULT NOW(),
    temperature float,
    health float,
    production_progress float,
    humidity float,
    fertility float,
    rentability float,
    pack_id varchar(200),
    FOREIGN KEY (pack_id) REFERENCES sensor_pack(id)
);

-- Create AlertType and LevelType enums
CREATE TYPE alert_type AS ENUM ('Health', 'Production', 'Rentability','Humidity','Fertility','Other');
CREATE TYPE level_type AS ENUM ('Low','Medium','High','Urgent');

-- Create Alert table
CREATE TABLE alert (
    id SERIAL PRIMARY KEY,
    date timestamp DEFAULT NOW(),
    title varchar(200) NOT NULL,
    description varchar(200) NOT NULL,
    type alert_type not null,
    level level_type DEFAULT 'Low',
    recommandation varchar(200),
    is_seen boolean DEFAULT false,
    state_id int,
    FOREIGN KEY (state_id) REFERENCES state(id)
);

-- Create Planning table
CREATE TABLE planning (
    id SERIAL PRIMARY KEY,
    date timestamp DEFAULT NOW(),
    title varchar(200) NOT NULL,
    description varchar(200) NOT NULL,
    start_date timestamp,
    end_date timestamp,
    ground int,
    FOREIGN KEY (ground) REFERENCES ground(id)
);

