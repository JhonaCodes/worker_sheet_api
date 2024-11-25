-- Luego las tablas en orden de dependencia
\i /docker-entrypoint-initdb.d/users.sql
\i /docker-entrypoint-initdb.d/activities.sql
\i /docker-entrypoint-initdb.d/activity_photos.sql
\i /docker-entrypoint-initdb.d/participants.sql