use database::db_error::DatabaseError;
use std::num::ParseIntError;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub code: i64,
    pub message: String,
}

impl From<DatabaseError> for ErrorResponse {
    fn from(item: DatabaseError) -> Self {
        ErrorResponse { code: 1, message: item.detail() }
    }
}

impl From<ParseIntError> for ErrorResponse {
    fn from(item: ParseIntError) -> Self {
        ErrorResponse { code: 2, message: item.to_string() }
    }
}