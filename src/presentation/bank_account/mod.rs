use super::*;
use cqrs_es::persist::ViewRepository;
use cqrs_es::Query;
use cqrs_es::{persist::GenericQuery, EventEnvelope, View};

use crate::application::{BankAccountServices, HappyPathBankAccountServices};
use std::sync::Arc;

use async_trait::async_trait;
use cfg_if::cfg_if;

pub mod bank_account_graphql;
pub mod bank_account_handlers;
pub mod bank_account_views;

// Re-exports

pub use bank_account_graphql::*;
pub use bank_account_handlers::*;
pub use bank_account_views::*;

cfg_if! {
    if #[cfg(feature = "postgres")] {
        use postgres_es::{PostgresCqrs, PostgresViewRepository};
        use sqlx::{Pool, Postgres};
        pub fn get_bank_account_cqrs_framework(
            pool: Pool<Postgres>,
        ) -> (
            Arc<PostgresCqrs<BankAccount>>,
            Arc<PostgresViewRepository<BankAccountView, BankAccount>>,
        ) {
            // A very simple query that writes each event to stdout.
            let simple_query = SimpleLoggingQuery {};

            // A query that stores the current state of an individual account.
            let account_view_repo = Arc::new(PostgresViewRepository::new("account_query", pool.clone()));
            let mut account_query = AccountQuery::new(account_view_repo.clone());

            // Without a query error handler there will be no indication if an
            // error occurs (e.g., database connection failure, missing columns or table).
            // Consider logging an error or panicking in your own application.
            account_query.use_error_handler(Box::new(|e| println!("{}", e)));

            // Create and return an event-sourced `CqrsFramework`.
            let queries: Vec<Box<dyn Query<BankAccount>>> =
                vec![Box::new(simple_query), Box::new(account_query)];
            let services = BankAccountServices::new(Box::new(HappyPathBankAccountServices));
            (
                Arc::new(postgres_es::postgres_cqrs(pool, queries, services)),
                account_view_repo,
            )
        }
    } else if #[cfg(feature = "mysql")] {
        use sqlx::{Pool, MySql};
        use mysql_es::{MysqlCqrs, MysqlViewRepository};

        pub fn get_bank_account_cqrs_framework(
            pool: Pool<MySql>,
        ) -> (
            Arc<MysqlCqrs<BankAccount>>,
            Arc<MysqlViewRepository<BankAccountView, BankAccount>>,
        ) {
            // A very simple query that writes each event to stdout.
            let simple_query = SimpleLoggingQuery {};

            // A query that stores the current state of an individual account.
            let account_view_repo = Arc::new(MysqlViewRepository::new("account_query", pool.clone()));
            let mut account_query = AccountQuery::new(account_view_repo.clone());

            // Without a query error handler there will be no indication if an
            // error occurs (e.g., database connection failure, missing columns or table).
            // Consider logging an error or panicking in your own application.
            account_query.use_error_handler(Box::new(|e| println!("{}", e)));

            // Create and return an event-sourced `CqrsFramework`.
            let queries: Vec<Box<dyn Query<BankAccount>>> =
                vec![Box::new(simple_query), Box::new(account_query)];
            let services = BankAccountServices::new(Box::new(HappyPathBankAccountServices));
            (
                Arc::new(mysql_es::mysql_cqrs(pool, queries, services)),
                account_view_repo,
            )
        }
    } else {
        compile_error!("Must specify either mysql or postgres feature");
    }
}
