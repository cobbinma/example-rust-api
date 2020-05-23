use postgres::Error as PostgresError;
use refinery::Error as RefineryError;
use sqlx::Error as SQLXError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DatabaseError {
    details: String,
}

impl DatabaseError {
    fn new(msg: &str) -> DatabaseError {
        DatabaseError {
            details: msg.to_string(),
        }
    }

    pub fn detail(&self) -> String {
        self.details.clone()
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for DatabaseError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<PostgresError> for DatabaseError {
    fn from(err: PostgresError) -> Self {
        DatabaseError::new(&err.to_string())
    }
}

impl From<RefineryError> for DatabaseError {
    fn from(err: RefineryError) -> Self {
        DatabaseError::new(&err.to_string())
    }
}

impl From<SQLXError> for DatabaseError {
    fn from(err: SQLXError) -> Self {
        DatabaseError::new(&err.to_string())
    }
}

impl From<std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>>
    for DatabaseError
{
    fn from(
        err: std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>,
    ) -> Self {
        DatabaseError::new(&err.to_string())
    }
}
