#![allow(unused)]
#![allow(clippy::all)]

use bigdecimal::BigDecimal;
use chrono::offset::Utc;
use chrono::DateTime;
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable, Identifiable, Debug)]
#[diesel(table_name = crate::database::schema::double_chess_games)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DoubleChessGame {
    pub id: Uuid,
    pub game_link: String,
    pub date: DateTime<Utc>,
    pub white_team: Vec<Option<Uuid>>,
    pub black_team: Vec<Option<Uuid>>,
    pub result: i32,
}

#[derive(Queryable, Insertable, Selectable, Identifiable, Debug)]
#[diesel(table_name = crate::database::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub password: String,
    pub email: String,
    pub name: String,
    pub registration_date: DateTime<Utc>,
    pub double_chess_bullet_rating: BigDecimal,
    pub double_chess_blitz_rating: BigDecimal,
    pub double_chess_rapid_rating: BigDecimal,
    pub double_chess_classical_rating: BigDecimal,
}
