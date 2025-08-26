CREATE TABLE Message (
    id SERIAL PRIMARY KEY,
    sender_id int,
    content varchar(200),
    discussion_id int,
    FOREIGN KEY (sender_id) REFERENCES Users(user_id),
    FOREIGN KEY (discussion_id) REFERENCES Discussion(id)
);

