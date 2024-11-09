create extension if not exists "uuid-ossp";

create table if not exists activities (
    id uuid not null primary key default uuid_generate_v4(),
    user_id text not null,
    title text not null,
    description text not null,
    status text not null,
    risk_level text not null,
    location_lat double precision,
    location_lng double precision,
    start_date timestamp,
    end_date timestamp,
    created_at timestamp,
    updated_at timestamp,
    is_synchronized boolean not null,
    hashtag text,
    is_deleted boolean
);


