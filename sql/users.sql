create table if not exists users (
    id uuid not null primary key,
    first_name text not null,
    last_name text not null,
    email text not null unique,
    password_hash text not null,
    position text not null,
    department text not null,
    phone text not null,
    status text not null,
    email_notification boolean,
    push_notification boolean,
    auto_sync boolean,
    created_at timestamp,
    hash_sync text
);

