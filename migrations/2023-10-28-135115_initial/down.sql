-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS `users`;

DROP TABLE IF EXISTS `roles`;

DROP TABLE IF EXISTS `properties`;

DROP TABLE IF EXISTS `machines`;

DROP TABLE IF EXISTS `reservations`;

DROP EXTENSION IF EXISTS "uuid-ossp";