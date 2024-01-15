-- Add up migration script here
create table if not exists todos (
    "id"  serial primary key,
    "title" varchar(100) not null,
    "completed" bool
)