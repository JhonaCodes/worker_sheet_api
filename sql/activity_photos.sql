create table if not exists activity_photos (
    id serial primary key ,
    activity_id text not null,
    url text not null
);