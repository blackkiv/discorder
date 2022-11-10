pub fn drop_create_tables() -> String {
    "
drop table if exists accepted_languages;

drop table if exists accepted_languages_weighted;

drop table if exists account;

drop table if exists relationship;

drop table if exists channel;

drop table if exists channel_recipient;

drop table if exists message;

drop table if exists activity;

drop table if exists server;

create table account (
    id text primary key not null,
    username text not null,
    discriminator integer,
    email text not null,
    verified boolean not null,
    avatar_hash text not null,
    has_mobile boolean not null,
    needs_email_verification boolean not null,
    premium_until text,
    flags integer not null,
    phone text,
    temp_banned_until text,
    ip text not null,
    user_profile_metadata_id integer,
    boosting_started_at text,
    premium_started_at text
);

create table relationship(
    id text primary key not null,
    account_id text not null,
    relation_type integer not null,
    nickname text,
    username text not null,
    avatar text,
    avatar_decoration text,
    discriminator text not null,
    public_flags integer not null,
    foreign key (account_id) references account (id) on delete cascade
);

create table server(id text primary key not null, name text not null);

create table channel(
    id text primary key not null,
    type integer not null,
    server_id text -- cannot be foreign key because it's nullable
);

create table message(
    id text primary key not null,
    channel_id text not null,
    timestamp text not null,
    contents text,
    attachments text,
    foreign key (channel_id) references channel (id) on delete cascade
);

create table channel_recipient(
    id integer not null primary key autoincrement,
    channel_id text not null,
    recipient text not null,
    foreign key (channel_id) references channel (id) on delete cascade
);

create table activity(
    event_id text not null primary key,
    event_type text not null,
    user_id text not null,
    domain text not null,
    client_send_timestamp text not null,
    client_track_timestamp text not null,
    timestamp text,
    other blob not null,
    foreign key (user_id) references account (id) on delete cascade
);

create table accepted_languages(
    id integer not null primary key autoincrement,
    event_id text not null,
    language text not null,
    foreign key (event_id) references activity (id) on delete cascade
);

create table accepted_languages_weighted(
    id integer not null primary key autoincrement,
    event_id text not null,
    language text not null,
    foreign key (event_id) references activity (id) on delete cascade
);
    "
    .to_string()
}
