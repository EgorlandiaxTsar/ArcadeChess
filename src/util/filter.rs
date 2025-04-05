use chrono::{DateTime, Utc};

pub type DateTimeFilter = Filter<DateTime<Utc>>;

pub type U8Filter = Filter<u8>;
pub type U16Filter = Filter<u16>;
pub type U32Filter = Filter<u32>;
pub type U64Filter = Filter<u64>;
pub type U128Filter = Filter<u128>;

pub type I8Filter = Filter<i8>;
pub type I16Filter = Filter<i16>;
pub type I32Filter = Filter<i32>;
pub type I64Filter = Filter<i64>;
pub type I128Filter = Filter<i128>;

pub type F32Filter = Filter<f32>;
pub type F64Filter = Filter<f64>;

pub enum Filter<T> {
    Lower(T),
    LowerOrEqual(T),
    Equal(T),
    HigherOrEqual(T),
    Higher(T),
    Ranged(T, T),
}

impl<T: PartialOrd + PartialEq> Filter<T> {
    pub fn apply(&self, value: &T) -> bool {
        match self {
            Filter::Lower(x) => value < x,
            Filter::LowerOrEqual(x) => value <= x,
            Filter::Equal(x) => value == x,
            Filter::HigherOrEqual(x) => value >= x,
            Filter::Higher(x) => value > x,
            Filter::Ranged(min, max) => value >= min && value <= max,
        }
    }
}
