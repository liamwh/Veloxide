use std::sync::Arc;

use crate::prelude::Result;
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[async_trait]
#[cfg_attr(test, automock)]
pub trait TodoRepository {
    async fn get_all(&self) -> Vec<Todo>;
    async fn get_todo_by_id(&self, id: &i32) -> Result<Todo>;
    async fn create_todo(&self, todo: &Todo) -> Result<Todo>;
    async fn delete_todo(&self, id: &i32) -> Result<()>;
    async fn update_todo(&self, todo: &Todo) -> Result<Todo>;
}

pub type DynTodoRepo = Arc<dyn TodoRepository + Send + Sync>;

#[derive(Serialize, Deserialize, ToSchema, Clone, Copy, Debug, PartialEq, sqlx::Type, Default)]
#[sqlx(type_name = "status", rename_all = "lowercase")]
pub enum Status {
    Complete,
    InProgress,
    #[default]
    NotStarted,
}

impl From<&str> for Status {
    fn from(status: &str) -> Self {
        match status {
            "Complete" => Status::Complete,
            "InProgress" => Status::InProgress,
            "NotStarted" => Status::NotStarted,
            _ => Status::NotStarted,
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Complete => "Complete".to_string(),
            Status::InProgress => "InProgress".to_string(),
            Status::NotStarted => "NotStarted".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, FromRow, PartialEq)]
pub struct Todo {
    pub id: i32,

    #[schema(example = "Get started building my new API!")]
    pub description: String,

    #[schema(example = "NotStarted")]
    pub done: bool,
}

impl Default for Todo {
    fn default() -> Self {
        Self::new(0, "Default description".to_string(), false)
    }
}

#[cfg_attr(test, automock)]
impl Todo {
    pub fn new(id: i32, description: String, status: bool) -> Self {
        Self {
            id,
            description,
            done: status,
        }
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn set_status(&mut self, status: bool) {
        self.done = status;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn todo_set_status_to_complete() {
        let mut todo = Todo::default();
        todo.set_status(true);
        assert_eq!(todo.done, true);
    }

    #[tokio::test]
    async fn todo_set_status_to_not_started() {
        let mut todo = Todo::new(1, "Get started building my new API!".to_string(), true);
        todo.set_status(false);
        assert_eq!(todo.done, false);
    }

    #[tokio::test]
    async fn todo_set_description() {
        let mut todo = Todo::new(1, "Get started building my new API!".to_string(), false);
        todo.set_description("Get started building my new API! (updated)".to_string());
        assert_eq!(
            todo.description,
            "Get started building my new API! (updated)"
        );
    }

    #[tokio::test]
    async fn status_from_str() {
        let status = Status::from("Complete");
        assert_eq!(status, Status::Complete);

        let status = Status::from("InProgress");
        assert_eq!(status, Status::InProgress);

        let status = Status::from("NotStarted");
        assert_eq!(status, Status::NotStarted);
    }

    #[tokio::test]
    async fn status_to_string() {
        let status = Status::Complete.to_string();
        assert_eq!(status, "Complete");

        let status = Status::InProgress.to_string();
        assert_eq!(status, "InProgress");

        let status = Status::NotStarted.to_string();
        assert_eq!(status, "NotStarted");
    }
}
