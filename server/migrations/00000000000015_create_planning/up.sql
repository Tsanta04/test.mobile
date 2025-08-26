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

