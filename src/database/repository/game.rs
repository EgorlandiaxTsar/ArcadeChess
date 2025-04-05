use crate::database::contract::{CrudSupport, OrderSupport};
use crate::database::model::entity::DoubleChessGame;
use crate::util::filter::DateTimeFilter;
use diesel::backend::Backend;
use diesel::expression::{NonAggregate, ValidGrouping};
use diesel::query_builder::{QueryFragment, QueryId};
use diesel::{BoxableExpression, Connection, QueryResult, SelectableExpression};
use uuid::Uuid;

pub trait DoubleChessGameModelApi: CrudSupport<DoubleChessGame, Self::ConnectionManager> {
    type ConnectionManager: Connection;
    type ConnectionBackend: Backend;

    fn filter_by_date<O>(
        &self,
        conn: &mut Self::ConnectionManager,
        filter: DateTimeFilter,
        order: Option<O>,
        limit: Option<u64>,
    ) -> QueryResult<Vec<DoubleChessGame>>
    where
        O: BoxableExpression<
                crate::database::schema::double_chess_games::table,
                Self::ConnectionBackend,
            > + ValidGrouping<()>
            + NonAggregate
            + QueryId
            + Copy
            + QueryFragment<Self::ConnectionBackend>
            + SelectableExpression<crate::database::schema::double_chess_games::table>,
        O::SqlType: OrderSupport;

    fn filter_by_result<O>(
        &self,
        conn: &mut Self::ConnectionManager,
        result: i8,
        order: Option<O>,
        limit: Option<u64>,
    ) -> QueryResult<Vec<DoubleChessGame>>
    where
        O: BoxableExpression<
                crate::database::schema::double_chess_games::table,
                Self::ConnectionBackend,
            > + ValidGrouping<()>
            + NonAggregate
            + QueryId
            + Copy
            + QueryFragment<Self::ConnectionBackend>,
        O::SqlType: OrderSupport;

    fn filter_by_teams<O>(
        &self,
        conn: &mut Self::ConnectionManager,
        white: &[Option<Uuid>; 2],
        black: &[Option<Uuid>; 2],
        order: Option<O>,
        limit: Option<u64>,
    ) -> QueryResult<Vec<DoubleChessGame>>
    where
        O: BoxableExpression<
                crate::database::schema::double_chess_games::table,
                Self::ConnectionBackend,
            > + ValidGrouping<()>
            + NonAggregate
            + QueryId
            + Copy
            + QueryFragment<Self::ConnectionBackend>,
        O::SqlType: OrderSupport;
}
