use crate::database::model::{DoubleChessGame, User};
use crate::database::schema::double_chess_games::result;
use crate::util::filters::{DateTimeFilter, F32Filter};
use diesel::{Connection, PgConnection};
use uuid::Uuid;

pub struct PostgresUserRepo;

pub struct PostgresDoubleChessGameRepo;

impl UserRepo for PostgresUserRepo {
    type ConnectionManager = PgConnection;

    fn find_by_uuid(&self, conn: &mut Self::ConnectionManager, uuid: Uuid) -> Option<User> {
        todo!()
    }

    fn find_by_email(&self, conn: &mut Self::ConnectionManager, email: &str) -> Option<User> {
        todo!()
    }

    fn find_by_name(&self, conn: &mut Self::ConnectionManager, name: &str) -> Option<User> {
        todo!()
    }

    fn filter_by_regdate(
        &self,
        conn: &mut Self::ConnectionManager,
        filter: DateTimeFilter,
        limit: Option<u64>,
    ) -> Vec<User> {
        todo!()
    }

    fn filter_by_rating(
        &self,
        conn: &mut Self::ConnectionManager,
        filter: F32Filter,
        limit: Option<u64>,
    ) -> Vec<User> {
        todo!()
    }

    fn filter_by_ratings(
        &self,
        conn: &mut Self::ConnectionManager,
        filter: &[Option<F32Filter>],
        limit: Option<u64>,
    ) -> Vec<User> {
        todo!()
    }
}

impl DoubleChessGameRepo for PostgresDoubleChessGameRepo {
    type ConnectionManager = PgConnection;

    fn find_by_uuid(
        &self,
        conn: &mut Self::ConnectionManager,
        uuid: Uuid,
    ) -> Option<DoubleChessGame> {
        todo!()
    }

    fn filter_by_date(
        &self,
        conn: &mut Self::ConnectionManager,
        filter: DateTimeFilter,
        limit: Option<u64>,
    ) -> Vec<DoubleChessGame> {
        todo!()
    }

    fn filter_by_result(
        &self,
        conn: &mut Self::ConnectionManager,
        i8: result,
        limit: Option<u64>,
    ) -> Vec<DoubleChessGame> {
        todo!()
    }

    fn filter_by_teams(
        &self,
        conn: &mut Self::ConnectionManager,
        white: &[Option<Uuid>; 2],
        black: &[Option<Uuid>; 2],
        limit: Option<u64>,
    ) -> Vec<DoubleChessGame> {
        todo!()
    }
}

pub trait UserRepo {
    type ConnectionManager: Connection;

    fn find_by_uuid(&self, conn: &mut Self::ConnectionManager, uuid: Uuid) -> Option<User>;

    fn find_by_email(&self, conn: &mut Self::ConnectionManager, email: &str) -> Option<User>;

    fn find_by_name(&self, conn: &mut Self::ConnectionManager, name: &str) -> Option<User>;

    fn filter_by_regdate(
        &self,
        conn: &mut Self::ConnectionManager,
        filter: DateTimeFilter,
        limit: Option<u64>,
    ) -> Vec<User>;

    fn filter_by_rating(
        &self,
        conn: &mut Self::ConnectionManager,
        filter: F32Filter,
        limit: Option<u64>,
    ) -> Vec<User>;

    fn filter_by_ratings(
        &self,
        conn: &mut Self::ConnectionManager,
        filter: &[Option<F32Filter>],
        limit: Option<u64>,
    ) -> Vec<User>;
}

pub trait DoubleChessGameRepo {
    type ConnectionManager: Connection;

    fn find_by_uuid(
        &self,
        conn: &mut Self::ConnectionManager,
        uuid: Uuid,
    ) -> Option<DoubleChessGame>;

    fn filter_by_date(
        &self,
        conn: &mut Self::ConnectionManager,
        filter: DateTimeFilter,
        limit: Option<u64>,
    ) -> Vec<DoubleChessGame>;

    fn filter_by_result(
        &self,
        conn: &mut Self::ConnectionManager,
        i8: result,
        limit: Option<u64>,
    ) -> Vec<DoubleChessGame>;

    fn filter_by_teams(
        &self,
        conn: &mut Self::ConnectionManager,
        white: &[Option<Uuid>; 2],
        black: &[Option<Uuid>; 2],
        limit: Option<u64>,
    ) -> Vec<DoubleChessGame>;
}
