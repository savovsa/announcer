-- Add migration script here
CREATE TABLE sound (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  file_name TEXT NOT NULL,
  display_name TEXT NOT NULL
);

INSERT INTO sound (file_name, display_name)
VALUES ("soft_bells.mp3", "Soft bells"); 