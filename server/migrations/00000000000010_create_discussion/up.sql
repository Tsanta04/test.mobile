CREATE TABLE Discussion (
    id SERIAL PRIMARY KEY,
    name varchar(200) NOT NULL,
    initialised_at timestamp DEFAULT NOW()
);

