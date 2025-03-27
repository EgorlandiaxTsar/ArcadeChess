// @generated automatically by Diesel CLI.

diesel::table! {
    double_chess_games (id) {
        id -> Uuid,
        #[max_length = 128]
        game_link -> Varchar,
        date -> Timestamptz,
        white_team -> Array<Nullable<Uuid>>,
        black_team -> Array<Nullable<Uuid>>,
        result -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 32]
        password -> Varchar,
        #[max_length = 64]
        email -> Varchar,
        #[max_length = 32]
        name -> Varchar,
        registration_date -> Timestamptz,
        double_chess_bullet_rating -> Numeric,
        double_chess_blitz_rating -> Numeric,
        double_chess_rapid_rating -> Numeric,
        double_chess_classical_rating -> Numeric,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    double_chess_games,
    users,
);
