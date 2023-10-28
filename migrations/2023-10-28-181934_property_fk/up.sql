-- Your SQL goes here
ALTER TABLE users ADD FOREIGN KEY (property) REFERENCES properties (id);