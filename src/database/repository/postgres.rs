use crate::database::contract::{CrudSupport, OrderSupport, PatchSupport, RatingFilterSupport};
use crate::database::model::entity::{DoubleChessGame, User};
use crate::database::repository::game::DoubleChessGameModelApi;
use crate::database::repository::user::UserModelApi;
use crate::database::schema::double_chess_games as double_chess_games_schema;
use crate::database::schema::users as users_schema;
use crate::util::filter::{DateTimeFilter, F32Filter};
use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::expression::is_aggregate::No;
use diesel::expression::{MixedAggregates, NonAggregate, ValidGrouping};
use diesel::pg::Pg;
use diesel::query_builder::{QueryFragment, QueryId};
use diesel::sql_types::Numeric;
use diesel::{
    BoxableExpression, Expression, ExpressionMethods, PgConnection, QueryResult, RunQueryDsl,
};
use diesel::{QueryDsl, SelectableExpression};
use uuid::Uuid;

pub type DefaultConnectionManger = PgConnection;
pub type DefaultConnectionBackend = Pg;

pub struct PostgresUserRepository;
pub struct PostgresDoubleChessGameRepository;

impl CrudSupport<User, PgConnection> for PostgresUserRepository {
    fn create(&self, conn: &mut PgConnection, entity: &User) -> QueryResult<User> {
        diesel::insert_into(users_schema::table)
            .values(entity)
            .get_result(conn)
    }

    fn read(&self, conn: &mut PgConnection, uuid: Uuid) -> QueryResult<User> {
        users_schema::table
            .filter(users_schema::id.eq(uuid))
            .first::<User>(conn)
    }

    fn update(
        &self,
        conn: &mut PgConnection,
        uuid: Uuid,
        entity: &<User as PatchSupport>::PatchType,
    ) -> QueryResult<User> {
        let user = self.read(conn, uuid);
        match user {
            Ok(_) => {
                let mut user = user?;
                user.patch(entity);
                diesel::update(users_schema::table.find(uuid))
                    .set((
                        users_schema::password.eq(user.password),
                        users_schema::email.eq(user.email),
                        users_schema::name.eq(user.name),
                        users_schema::double_chess_bullet_rating
                            .eq(user.double_chess_bullet_rating),
                        users_schema::double_chess_blitz_rating.eq(user.double_chess_blitz_rating),
                        users_schema::double_chess_rapid_rating.eq(user.double_chess_rapid_rating),
                        users_schema::double_chess_classical_rating
                            .eq(user.double_chess_classical_rating),
                    ))
                    .get_result(conn)
            }
            Err(_) => user,
        }
    }

    fn delete(&self, conn: &mut PgConnection, uuid: Uuid) -> QueryResult<usize> {
        diesel::delete(users_schema::table)
            .filter(users_schema::id.eq(uuid))
            .execute(conn)
    }
}

impl UserModelApi for PostgresUserRepository {
    type ConnectionManager = PgConnection;
    type ConnectionBackend = Pg;

    fn find_by_email(&self, conn: &mut Self::ConnectionManager, email: &str) -> QueryResult<User> {
        users_schema::table
            .filter(users_schema::email.eq(email))
            .first::<User>(conn)
    }

    fn find_by_name(&self, conn: &mut Self::ConnectionManager, name: &str) -> QueryResult<User> {
        users_schema::table
            .filter(users_schema::name.eq(name))
            .first::<User>(conn)
    }

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
        O::SqlType: OrderSupport,
    {
        let mut query = match filter {
            DateTimeFilter::Lower(x) => users_schema::table
                .filter(users_schema::registration_date.lt(x))
                .into_boxed(),
            DateTimeFilter::LowerOrEqual(x) => users_schema::table
                .filter(users_schema::registration_date.le(x))
                .into_boxed(),
            DateTimeFilter::Equal(x) => users_schema::table
                .filter(users_schema::registration_date.eq(x))
                .into_boxed(),
            DateTimeFilter::HigherOrEqual(x) => users_schema::table
                .filter(users_schema::registration_date.ge(x))
                .into_boxed(),
            DateTimeFilter::Higher(x) => users_schema::table
                .filter(users_schema::registration_date.gt(x))
                .into_boxed(),
            DateTimeFilter::Ranged(min, max) => users_schema::table
                .filter(users_schema::registration_date.between(min, max))
                .into_boxed(),
        };
        query = if let Some(o) = order {
            query.order_by(o)
        } else {
            query.order_by(users_schema::registration_date)
        };
        if limit == Some(0) {
            query = query.limit(limit.unwrap() as i64)
        };
        query.get_results(conn)
    }

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
        O::SqlType: OrderSupport,
    {
        let mut query = match filter {
            F32Filter::Lower(x) => users_schema::table
                .filter(
                    users_schema::double_chess_bullet_rating.lt(BigDecimal::from_f32(x).unwrap()),
                )
                .into_boxed(),
            F32Filter::LowerOrEqual(x) => users_schema::table
                .filter(
                    users_schema::double_chess_bullet_rating.le(BigDecimal::from_f32(x).unwrap()),
                )
                .into_boxed(),
            F32Filter::Equal(x) => users_schema::table
                .filter(
                    users_schema::double_chess_bullet_rating.eq(BigDecimal::from_f32(x).unwrap()),
                )
                .into_boxed(),
            F32Filter::HigherOrEqual(x) => users_schema::table
                .filter(
                    users_schema::double_chess_bullet_rating.ge(BigDecimal::from_f32(x).unwrap()),
                )
                .into_boxed(),
            F32Filter::Higher(x) => users_schema::table
                .filter(
                    users_schema::double_chess_bullet_rating.gt(BigDecimal::from_f32(x).unwrap()),
                )
                .into_boxed(),
            F32Filter::Ranged(min, max) => users_schema::table
                .filter(users_schema::double_chess_bullet_rating.between(
                    BigDecimal::from_f32(min).unwrap(),
                    BigDecimal::from_f32(max).unwrap(),
                ))
                .into_boxed(),
        };
        query = if let Some(o) = order {
            query.order_by(o)
        } else {
            query.order_by(users_schema::double_chess_bullet_rating)
        };
        if limit == Some(0) {
            query = query.limit(limit.unwrap() as i64)
        };
        query.get_results(conn)
    }

    fn delete_by_email(
        &self,
        conn: &mut Self::ConnectionManager,
        email: &str,
    ) -> QueryResult<usize> {
        diesel::delete(users_schema::table)
            .filter(users_schema::email.eq(email))
            .execute(conn)
    }

    fn delete_by_name(&self, conn: &mut Self::ConnectionManager, name: &str) -> QueryResult<usize> {
        diesel::delete(users_schema::table)
            .filter(users_schema::name.eq(name))
            .execute(conn)
    }
}

impl CrudSupport<DoubleChessGame, PgConnection> for PostgresDoubleChessGameRepository {
    fn create(
        &self,
        conn: &mut PgConnection,
        entity: &DoubleChessGame,
    ) -> QueryResult<DoubleChessGame> {
        diesel::insert_into(double_chess_games_schema::table)
            .values(entity)
            .get_result(conn)
    }

    fn read(&self, conn: &mut PgConnection, uuid: Uuid) -> QueryResult<DoubleChessGame> {
        double_chess_games_schema::table
            .filter(double_chess_games_schema::id.eq(uuid))
            .first::<DoubleChessGame>(conn)
    }

    /// Since `DoubleChessGame` entities do not support patching, but `update` method is required for `CrudSupport<E, M>`, this function automatically panics when called as there is no sense to use it.
    fn update(
        &self,
        conn: &mut PgConnection,
        uuid: Uuid,
        entity: &<DoubleChessGame as PatchSupport>::PatchType,
    ) -> QueryResult<DoubleChessGame> {
        panic!("Operation is not supported for this repository")
    }

    fn delete(&self, conn: &mut PgConnection, uuid: Uuid) -> QueryResult<usize> {
        diesel::delete(double_chess_games_schema::table)
            .filter(double_chess_games_schema::id.eq(uuid))
            .execute(conn)
    }
}

impl DoubleChessGameModelApi for PostgresDoubleChessGameRepository {
    type ConnectionManager = DefaultConnectionManger;
    type ConnectionBackend = DefaultConnectionBackend;

    fn filter_by_date<O>(
        &self,
        conn: &mut Self::ConnectionManager,
        filter: DateTimeFilter,
        order: Option<O>,
        limit: Option<u64>,
    ) -> QueryResult<Vec<DoubleChessGame>>
    where
        O: BoxableExpression<double_chess_games_schema::table, Self::ConnectionBackend>
            + ValidGrouping<()>
            + NonAggregate
            + QueryId
            + Copy
            + QueryFragment<Self::ConnectionBackend>
            + SelectableExpression<double_chess_games_schema::table>,
        O::SqlType: OrderSupport,
    {
        let mut query = match filter {
            DateTimeFilter::Lower(x) => double_chess_games_schema::table
                .filter(double_chess_games_schema::date.lt(x))
                .into_boxed(),
            DateTimeFilter::LowerOrEqual(x) => double_chess_games_schema::table
                .filter(double_chess_games_schema::date.le(x))
                .into_boxed(),
            DateTimeFilter::Equal(x) => double_chess_games_schema::table
                .filter(double_chess_games_schema::date.eq(x))
                .into_boxed(),
            DateTimeFilter::HigherOrEqual(x) => double_chess_games_schema::table
                .filter(double_chess_games_schema::date.ge(x))
                .into_boxed(),
            DateTimeFilter::Higher(x) => double_chess_games_schema::table
                .filter(double_chess_games_schema::date.gt(x))
                .into_boxed(),
            DateTimeFilter::Ranged(min, max) => double_chess_games_schema::table
                .filter(double_chess_games_schema::date.between(min, max))
                .into_boxed(),
        };
        query = if let Some(o) = order {
            query.order_by(o)
        } else {
            query.order_by(double_chess_games_schema::date)
        };
        if limit == Some(0) {
            query = query.limit(limit.unwrap() as i64)
        }
        query.get_results(conn)
    }

    fn filter_by_result<O>(
        &self,
        conn: &mut Self::ConnectionManager,
        result: i8,
        order: Option<O>,
        limit: Option<u64>,
    ) -> QueryResult<Vec<DoubleChessGame>>
    where
        O: BoxableExpression<double_chess_games_schema::table, Self::ConnectionBackend>
            + ValidGrouping<()>
            + NonAggregate
            + QueryId
            + Copy
            + QueryFragment<Self::ConnectionBackend>,
        O::SqlType: OrderSupport,
    {
        let mut query = double_chess_games_schema::table
            .filter(double_chess_games_schema::result.eq(result as i32))
            .into_boxed();
        query = if let Some(o) = order {
            query.order_by(o)
        } else {
            query.order_by(double_chess_games_schema::date)
        };
        if limit == Some(0) {
            query = query.limit(limit.unwrap() as i64)
        }
        query.get_results(conn)
    }

    fn filter_by_teams<O>(
        &self,
        conn: &mut Self::ConnectionManager,
        white: &[Option<Uuid>; 2],
        black: &[Option<Uuid>; 2],
        order: Option<O>,
        limit: Option<u64>,
    ) -> QueryResult<Vec<DoubleChessGame>>
    where
        O: BoxableExpression<double_chess_games_schema::table, Self::ConnectionBackend>
            + ValidGrouping<()>
            + NonAggregate
            + QueryId
            + Copy
            + QueryFragment<Self::ConnectionBackend>,
        O::SqlType: OrderSupport,
    {
        todo!()
    }
}
