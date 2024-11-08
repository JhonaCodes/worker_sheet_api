
CREATE TABLE IF NOT EXISTS users (
                                     id TEXT PRIMARY KEY,
                                     first_name TEXT NOT NULL,
                                     last_name TEXT NOT NULL,
                                     email TEXT NOT NULL UNIQUE,
                                     password_hash TEXT NOT NULL,
                                     position TEXT NOT NULL,
                                     department TEXT NOT NULL,
                                     phone TEXT NOT NULL,
                                     status TEXT NOT NULL,
                                     email_notification BOOLEAN,
                                     push_notification BOOLEAN,
                                     auto_sync BOOLEAN,
                                     created_at TIMESTAMP WITHOUT TIME ZONE,
                                     updated_at TIMESTAMP WITHOUT TIME ZONE
);

CREATE TABLE IF NOT EXISTS activities (
                                          id TEXT PRIMARY KEY,
                                          title TEXT NOT NULL,
                                          description TEXT NOT NULL,
                                          status TEXT NOT NULL,
                                          risk_level TEXT NOT NULL,
                                          location_lat DOUBLE PRECISION,
                                          location_lng DOUBLE PRECISION,
                                          user_id TEXT NOT NULL,
                                          start_date TIMESTAMP,
                                          end_date TIMESTAMP,
                                          created_at TIMESTAMP,
                                          updated_at TIMESTAMP,
                                          is_synchronized BOOLEAN NOT NULL,
                                          hashtag TEXT,
                                          is_deleted BOOLEAN
);


CREATE TABLE IF NOT EXISTS activity_photos (
                                               id TEXT PRIMARY KEY,
                                               activity_id TEXT NOT NULL,
                                               url TEXT NOT NULL
);