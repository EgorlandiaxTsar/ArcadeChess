use chrono::{DateTime, Utc};

pub enum DateTimeFilter {
    GreaterThan(DateTime<Utc>),
    SmallerThan(DateTime<Utc>),
    RangedWithin(DateTime<Utc>, DateTime<Utc>)
}

pub enum I8Filter {
    GreaterThan(i8),
    SmallerThan(i8),
    RangedWithin(i8, i8)
}

pub enum I16Filter {
    GreaterThan(i16),
    SmallerThan(i16),
    RangedWithin(i16, i16)
}

pub enum I32Filter {
    GreaterThan(i32),
    SmallerThan(i32),
    RangedWithin(i32, i32)
}

pub enum I64Filter {
    GreaterThan(i64),
    SmallerThan(i64),
    RangedWithin(i64, i64)
}

pub enum I128Filter {
    GreaterThan(i128),
    SmallerThan(i128),
    RangedWithin(i128, i128)
}

pub enum F32Filter {
    GreaterThan(f32),
    SmallerThan(f32),
    RangedWithin(f32, f32)
}

pub enum F64Filter {
    GreaterThan(f64),
    SmallerThan(f64),
    RangedWithin(f64, f64)
}
