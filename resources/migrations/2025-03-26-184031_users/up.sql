-- Your SQL goes here
create table users
(
    id                            uuid                         not null
        constraint user_id_pk
            primary key,
    password                      varchar(32)                  not null,
    email                         varchar(64)                  not null,
    name                          varchar(32)                  not null,
    registration_date             timestamp(6) with time zone  not null,
    double_chess_bullet_rating    numeric(8, 4) default 1000.0 not null,
    double_chess_blitz_rating     numeric(8, 4) default 1000.0 not null,
    double_chess_rapid_rating     numeric(8, 4) default 1000.0 not null,
    double_chess_classical_rating numeric(8, 4) default 1000.0 not null
);

alter table users
    owner to postgres;

create index double_chess_blitz_rating_idx
    on users (double_chess_blitz_rating);

create index double_chess_bullet_rating_idx
    on users (double_chess_bullet_rating);

create index double_chess_classical_rating_idx
    on users (double_chess_classical_rating);

create index double_chess_rapid_rating_idx
    on users (double_chess_rapid_rating);

create index double_chess_rating_idx
    on users (double_chess_blitz_rating, double_chess_rapid_rating, double_chess_bullet_rating,
              double_chess_classical_rating);

create unique index email_idx
    on users (email);

create unique index name_idx
    on users (name);