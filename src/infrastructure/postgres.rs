use async_trait::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};
use sqlx::{Pool, Postgres};
use tracing::instrument;

use crate::domain::TodoRepository;

use super::Todo;

#[derive(Debug)]

pub struct PostgresTodoRepository {
    pool: Pool<Postgres>,
}

impl PostgresTodoRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
#[cfg_attr(test, automock)]
impl TodoRepository for PostgresTodoRepository {
    #[instrument]
    async fn get_all(&self) -> Vec<Todo> {
        sqlx::query_as!(Todo, "SELECT * FROM todos")
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    #[instrument]
    async fn get_todo_by_id(&self, id: &i32) -> crate::prelude::Result<Todo> {
        let todo = sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| super::Error::Generic(e.to_string()))?;

        Ok(todo)
    }

    #[instrument]
    async fn create_todo(&self, todo: &Todo) -> crate::prelude::Result<Todo> {
        let todo = sqlx::query_as!(
            Todo,
            "INSERT INTO todos (id, done, description) VALUES ($1, $2, $3) RETURNING *",
            &todo.id,
            &todo.done,
            &todo.description,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(todo)
    }

    #[instrument]
    async fn delete_todo(&self, id: &i32) -> crate::prelude::Result<()> {
        sqlx::query!("DELETE FROM todos WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(|e| super::Error::Generic(e.to_string()))?;

        Ok(())
    }

    #[instrument]
    async fn update_todo(&self, todo: &Todo) -> crate::prelude::Result<Todo> {
        let todo = sqlx::query_as!(
            Todo,
            "UPDATE todos SET description = $1, done = $2 WHERE id = $3 RETURNING *",
            &todo.description,
            &todo.done,
            &todo.id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| super::Error::Generic(e.to_string()))?;

        Ok(todo)
    }
}
