CREATE TABLE IF NOT EXISTS events
(
    id          INTEGER PRIMARY KEY NOT NULL,
    title       VARCHAR(255)        NOT NULL,
    start_date  INTEGER             NOT NULL,
    end_date    INTEGER             NOT NULL
);