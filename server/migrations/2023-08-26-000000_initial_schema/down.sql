-- Drop tables in reverse order to avoid foreign key constraints
DROP TABLE IF EXISTS planning;
DROP TABLE IF EXISTS alert;
DROP TYPE IF EXISTS level_type;
DROP TYPE IF EXISTS alert_type;
DROP TABLE IF EXISTS state;
DROP TABLE IF EXISTS participant;
DROP TABLE IF EXISTS message;
DROP TABLE IF EXISTS discussion;
DROP TABLE IF EXISTS ground;
DROP TABLE IF EXISTS pack;
DROP TABLE IF EXISTS sensor_pack;
DROP TABLE IF EXISTS sensor;
DROP TABLE IF EXISTS sensor_type;
DROP TABLE IF EXISTS location;
DROP TABLE IF EXISTS culture_type;
DROP TABLE IF EXISTS person;
DROP TABLE IF EXISTS users;

