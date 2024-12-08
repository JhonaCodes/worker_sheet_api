create table if not exists activity_photos (
    id serial primary key ,
    activity_id uuid not null,
    url text not null
);