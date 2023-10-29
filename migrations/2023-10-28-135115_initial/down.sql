-- This file should undo anything in up.sql
ALTER TABLE reservations DROP CONSTRAINT reservations_machine_fkey;
ALTER TABLE reservations DROP CONSTRAINT reservations_owner_fkey;
ALTER TABLE machines DROP CONSTRAINT machines_property_fkey;
ALTER TABLE users DROP CONSTRAINT users_role_fkey;
ALTER TABLE properties DROP CONSTRAINT properties_owner_fkey;

DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS roles;
DROP TABLE IF EXISTS properties;
DROP TABLE IF EXISTS machines;
DROP TABLE IF EXISTS reservations;

DROP EXTENSION IF EXISTS "uuid-ossp";