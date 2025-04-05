//! contract.rs
//!
//! This module contains generic trait definitions (aka "contracts") describing common hard-coded (but easily editable manually) behaviour across `database` module.
//!
//! # Traits:
//! - `CrudSupport`: Generic describing CRUD implementation for repositories.
//! - `OrderSupport`: This trait marks data type as valid for ordering in Diesel columns.
//! - `RatingFilterSupport`: This trait marks tables as valid for rating filtering on Diesel tables.
//! - `PatchSupport`: This trait marks Diesel SQL models as valid for patching and implements associated optional (flexible) type PatchType
//!
//! These traits allow flexible and type-safe interactions across models, queries, and database repositories.

use crate::database::model::entity::{DoubleChessGame, User};
use crate::database::model::updatable::{OptionalDoubleChessGame, OptionalUser};
use crate::database::schema::users as users_schema;
use diesel::sql_types::{Date, Numeric, Text, Timestamp, Timestamptz};
use diesel::{Connection, QueryResult};
use uuid::Uuid;

/// Generic describing CRUD implementation for repositories.
pub trait CrudSupport<E: PatchSupport, M: Connection> {
    fn create(&self, conn: &mut M, entity: &E) -> QueryResult<E>;

    fn read(&self, conn: &mut M, uuid: Uuid) -> QueryResult<E>;

    fn update(&self, conn: &mut M, uuid: Uuid, entity: &E::PatchType) -> QueryResult<E>;

    fn delete(&self, conn: &mut M, uuid: Uuid) -> QueryResult<usize>;
}

/// This trait marks data type as valid for ordering in Diesel columns.
pub trait OrderSupport {}

impl OrderSupport for Timestamptz {}
impl OrderSupport for Timestamp {}
impl OrderSupport for Date {}

impl OrderSupport for Numeric {}
impl OrderSupport for Text {}

impl OrderSupport for i8 {}
impl OrderSupport for i16 {}
impl OrderSupport for i32 {}
impl OrderSupport for i64 {}
impl OrderSupport for i128 {}

impl OrderSupport for u8 {}
impl OrderSupport for u16 {}
impl OrderSupport for u32 {}
impl OrderSupport for u64 {}
impl OrderSupport for u128 {}

impl OrderSupport for f32 {}
impl OrderSupport for f64 {}

/// This trait marks tables as valid for rating filtering on Diesel tables.
pub trait RatingFilterSupport {}

impl RatingFilterSupport for users_schema::double_chess_bullet_rating {}
impl RatingFilterSupport for users_schema::double_chess_blitz_rating {}
impl RatingFilterSupport for users_schema::double_chess_rapid_rating {}
impl RatingFilterSupport for users_schema::double_chess_classical_rating {}

/// This trait marks Diesel SQL models as valid for patching and implements associated optional (flexible) type `PatchType`
pub trait PatchSupport {
    type PatchType;

    fn patch(&mut self, entity: &Self::PatchType);
}

impl PatchSupport for User {
    type PatchType = OptionalUser;

    fn patch(&mut self, entity: &Self::PatchType) {
        if let Some(password) = &entity.password {
            self.password = password.clone();
        }
        if let Some(email) = &entity.email {
            self.email = email.clone();
        }
        if let Some(name) = &entity.name {
            self.name = name.clone();
        }
        if let Some(double_chess_bullet_rating) = &entity.double_chess_bullet_rating {
            self.double_chess_bullet_rating = double_chess_bullet_rating.clone();
        }
        if let Some(double_chess_blitz_rating) = &entity.double_chess_blitz_rating {
            self.double_chess_blitz_rating = double_chess_blitz_rating.clone();
        }
        if let Some(double_chess_rapid_rating) = &entity.double_chess_rapid_rating {
            self.double_chess_rapid_rating = double_chess_rapid_rating.clone();
        }
        if let Some(double_chess_classical_rating) = &entity.double_chess_classical_rating {
            self.double_chess_classical_rating = double_chess_classical_rating.clone();
        }
    }
}

impl PatchSupport for DoubleChessGame {
    type PatchType = OptionalDoubleChessGame;

    /// Since `DoubleChessGame` entities do not support patching, this function automatically panics when called as there is no sense to use it
    fn patch(&mut self, entity: &Self::PatchType) {
        panic!("Operation is not supported for this entity")
    }
}
