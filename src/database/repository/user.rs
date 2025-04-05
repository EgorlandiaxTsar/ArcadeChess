use crate::database::contract::{CrudSupport, OrderSupport, RatingFilterSupport};
use crate::database::model::entity::User;
use crate::database::schema::users as users_schema;
use crate::util::filter::{DateTimeFilter, F32Filter};
use diesel::backend::Backend;
use diesel::expression::is_aggregate::No;
use diesel::expression::{MixedAggregates, NonAggregate, ValidGrouping};
use diesel::query_builder::{QueryFragment, QueryId};
use diesel::sql_types::Numeric;
use diesel::{BoxableExpression, Connection, Expression, QueryResult};

pub trait UserModelApi: CrudSupport<User, Self::ConnectionManager> {
    type ConnectionManager: Connection;
    type ConnectionBackend: Backend;

    fn find_by_email(&self, conn: &mut Self::ConnectionManager, email: &str) -> QueryResult<User>;

    fn find_by_name(&self, conn: &mut Self::ConnectionManager, name: &str) -> QueryResult<User>;

    fn filter_by_regdate<O>(
        &self,
        conn: &mut Self::ConnectionManager,
        filter: DateTimeFilter,
        order: Option<O>,
        limit: Option<u64>,
    ) -> QueryResult<Vec<User>>
    where
        O: BoxableExpression<users_schema::table, Self::ConnectionBackend>
            + ValidGrouping<()>
            + NonAggregate
            + QueryId
            + Copy
            + QueryFragment<Self::ConnectionBackend>,
        O::SqlType: OrderSupport;

    fn filter_by_rating<O, C>(
        &self,
        conn: &mut Self::ConnectionManager,
        filter: F32Filter,
        order: Option<O>,
        limit: Option<u64>,
    ) -> QueryResult<Vec<User>>
    where
        C: BoxableExpression<users_schema::table, Self::ConnectionBackend, SqlType = Numeric>
            + Expression
            + ValidGrouping<()>
            + NonAggregate
            + QueryId
            + Copy
            + QueryFragment<Self::ConnectionBackend>
            + RatingFilterSupport,
        <C as ValidGrouping<()>>::IsAggregate: MixedAggregates<No, Output = No>,
        O: BoxableExpression<users_schema::table, Self::ConnectionBackend>
            + ValidGrouping<()>
            + NonAggregate
            + QueryId
            + Copy
            + QueryFragment<Self::ConnectionBackend>,
        O::SqlType: OrderSupport;

    fn delete_by_email(
        &self,
        conn: &mut Self::ConnectionManager,
        email: &str,
    ) -> QueryResult<usize>;

    fn delete_by_name(&self, conn: &mut Self::ConnectionManager, name: &str) -> QueryResult<usize>;
}
