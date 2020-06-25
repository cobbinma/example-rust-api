use database::db_error::DatabaseError;
use std::io::Error as IOError;
use std::num::ParseIntError;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub code: i64,
    pub message: String,
}

impl From<DatabaseError> for ErrorResponse {
    fn from(item: DatabaseError) -> Self {
        ErrorResponse {
            code: 1,
            message: format!("{} : {}", "database returned error", item.detail()),
        }
    }
}

impl From<ParseIntError> for ErrorResponse {
    fn from(item: ParseIntError) -> Self {
        ErrorResponse {
            code: 2,
            message: format!("{} : {}", "integer parsing error", item.to_string()),
        }
    }
}

impl From<IOError> for ErrorResponse {
    fn from(item: IOError) -> Self {
        ErrorResponse {
            code: 3,
            message: format!("{} : {}", "error parsing json body", item.to_string()),
        }
    }
}

impl From<Box<dyn std::error::Error>> for ErrorResponse {
    fn from(item: Box<dyn std::error::Error>) -> Self {
        ErrorResponse {
            code: 3,
            message: format!("{} : {}", "error parsing json body", item.to_string()),
        }
    }
}

impl From<http_types::Error> for ErrorResponse {
    fn from(item: http_types::Error) -> Self {
        ErrorResponse {
            code: 3,
            message: format!("{} : {}", "error parsing json body", item.to_string()),
        }
    }
}
