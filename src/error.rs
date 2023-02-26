//! This is the main (and only for now) application Error type.
//! It's using 'thiserror' as it reduces boilerplate error code while providing rich error typing.
//!
//! Notes:
//!     - The strategy is to start with one Error type for the whole application and then seggregate as needed.
//!     - Since everything is typed from the start, renaming and refactoring become relatively trivial.
//!     - By best practices, `anyhow` is not used in application code, but can be used in unit or integration test (will be in dev_dependencies when used)
//!

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic: {0}")]
    Generic(String), // The goal is to get rid of this generic error eventually

    #[error(transparent)]
    SetGlobalDefaultError(#[from] tracing::subscriber::SetGlobalDefaultError),

    #[error(transparent)]
    HyperError(#[from] hyper::Error),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    SetLoggerError(#[from] log::SetLoggerError),

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    SeaOrmDbError(#[from] sea_orm::DbErr),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_error() {
        let err = Error::Generic("test".to_string());
        assert_eq!(err.to_string(), "Generic: test");
    }
}
