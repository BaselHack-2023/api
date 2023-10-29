-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE roles (
    id UUID DEFAULT Uuid_generate_v4 (),
    name VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id)
);

CREATE TABLE users (
    id UUID DEFAULT Uuid_generate_v4 (),
    name VARCHAR NOT NULL,
    role UUID NOT NULL,
    property UUID,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id),
    FOREIGN KEY (role) REFERENCES roles (id)
);

CREATE TABLE properties (
    id UUID DEFAULT Uuid_generate_v4 (),
    name VARCHAR NOT NULL,
    address VARCHAR NOT NULL,
    address2 VARCHAR,
    city VARCHAR NOT NULL,
    zip VARCHAR NOT NULL,
    owner UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id),
    FOREIGN KEY (owner) REFERENCES users (id)
);

CREATE TABLE machines (
    id UUID DEFAULT Uuid_generate_v4 (),
    name VARCHAR NOT NULL,
    property UUID NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'stopped',
    eta TIMESTAMP NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id),
    FOREIGN KEY (property) REFERENCES properties (id)
);

CREATE TABLE reservations (
    id UUID DEFAULT Uuid_generate_v4 (),
    machine UUID NOT NULL,
    owner UUID NOT NULL,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    shared BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id),
    FOREIGN KEY (machine) REFERENCES machines (id),
    FOREIGN KEY (owner) REFERENCES users (id)
);