CREATE TABLE Participant (
    id SERIAL PRIMARY KEY,
    discussion_id int,
    participant_id int,
    FOREIGN KEY (participant_id) REFERENCES Users(user_id),
    FOREIGN KEY (discussion_id) REFERENCES Discussion(id)
);

