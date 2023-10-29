-- Your SQL goes here
CREATE TABLE items (
    id UUID DEFAULT Uuid_generate_v4 (),
    name VARCHAR NOT NULL,
    size VARCHAR NOT NULL,
    colors VARCHAR NOT NULL,
    owner UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id),
    FOREIGN KEY (owner) REFERENCES users (id)
);