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

