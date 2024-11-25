create table if not exists participants(
    id serial not null primary key,
    activity_id uuid not null,
    user_id uuid not null
)