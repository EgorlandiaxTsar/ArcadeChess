-- Your SQL goes here
create table double_chess_games
(
    id         uuid                        not null
        constraint double_chess_game_id_pk
            primary key,
    game_link  varchar(128)                not null,
    date       timestamp(6) with time zone not null,
    white_team uuid[]                      not null,
    black_team uuid[]                      not null,
    result     integer                     not null
);

alter table double_chess_games
    owner to postgres;

create index date_idx
    on double_chess_games (date);