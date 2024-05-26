use core::fmt;

pub enum DbError {
    Error(sqlx::Error),
    NoResult,
}
impl From<sqlx::Error> for DbError {
    fn from(value: sqlx::Error) -> Self {
        DbError::Error(value)
    }
}
impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::Error(e) => write!(f, "db error occured {:?}", e),
            DbError::NoResult => write!(f, "no response"),
        }
    }
}
