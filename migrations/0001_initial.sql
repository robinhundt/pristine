create table if not exists users (
    id integer primary key not null,
    name text not null
);

create table if not exists tasks (
    id integer primary key not null,
    name text not null,
    description text not null,
    duration integer not null
);

create table if not exists scheduled_tasks (
    id integer primary key not null,
    user integer not null,
    task integer not null,
    scheduled_at datetime not null,
    completed boolean not null default (false),
    foreign key (user) references users(id),
    foreign key (task) references tasks(id)
);