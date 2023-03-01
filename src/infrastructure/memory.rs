use crate::domain::{Todo, TodoRepository};
use async_trait::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};

use tokio::sync::Mutex;
use tracing::instrument;

/// In-memory todo store
pub type Store = Mutex<Vec<Todo>>;

#[derive(Debug)]
pub struct MemoryTodoRepository {
    store: Store,
}

impl MemoryTodoRepository {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(vec![]),
        }
    }
}

impl Default for MemoryTodoRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
#[cfg_attr(test, automock)]
impl TodoRepository for MemoryTodoRepository {
    #[instrument]
    async fn get_all(&self) -> Vec<Todo> {
        self.store.lock().await.clone()
    }

    #[instrument]
    async fn get_todo_by_id(&self, id: &i32) -> crate::prelude::Result<Todo> {
        self.store
            .lock()
            .await
            .iter()
            .find(|todo| &todo.id == id)
            .map(|todo| Ok(todo.clone()))
            .unwrap_or_else(|| {
                Err(super::Error::Generic(format!(
                    "todo with id ({}) not found",
                    id
                )))
            })
    }

    #[instrument]
    async fn create_todo(&self, todo: &Todo) -> crate::prelude::Result<Todo> {
        let mut todos = self.store.lock().await;

        todos
            .iter_mut()
            .find(|existing_todo| existing_todo.id == todo.id)
            .map(|found| {
                Err(super::Error::Generic(format!(
                    "todo with id ({}) already exists",
                    found.id
                )))
            })
            .unwrap_or_else(|| {
                todos.push(todo.clone());
                Ok(todo.clone())
            })
    }

    #[instrument]
    async fn delete_todo(&self, id: &i32) -> crate::prelude::Result<()> {
        let mut todos = self.store.lock().await;

        todos
            .iter()
            .position(|todo| todo.id == *id)
            .map(|index| {
                todos.remove(index);
                Ok(())
            })
            .unwrap_or_else(|| {
                Err(super::Error::Generic(format!(
                    "todo with id ({}) not found",
                    id
                )))
            })
    }

    #[instrument]
    async fn update_todo(&self, todo: &Todo) -> crate::prelude::Result<Todo> {
        let mut todos = self.store.lock().await;

        todos
            .iter_mut()
            .find(|existing_todo| existing_todo.id == todo.id)
            .map(|found| {
                found.set_description(todo.description.clone());
                found.set_status(todo.completed);
                Ok(found.clone())
            })
            .unwrap_or_else(|| {
                Err(super::Error::Generic(format!(
                    "todo with id ({}) not found",
                    todo.id
                )))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Todo;

    #[tokio::test]
    async fn test_get_all() {
        let repo = MemoryTodoRepository::new();
        let todos = repo.get_all().await;
        assert_eq!(todos.len(), 0);
    }

    #[tokio::test]
    async fn test_get_todo_by_id() {
        let repo = MemoryTodoRepository::new();
        let todo = Todo::new(1, "description 1".to_string(), false);
        repo.create_todo(&todo)
            .await
            .expect("Expected to create todo successfully");
        let found = repo
            .get_todo_by_id(&todo.id)
            .await
            .expect("Expected to get todo successfully");
        assert_eq!(todo, found);
    }

    #[tokio::test]
    async fn test_create_todo() {
        let repo = MemoryTodoRepository::new();
        let todo = Todo::new(1, "description 1".to_string(), false);
        repo.create_todo(&todo)
            .await
            .expect("Expected to create todo successfully");
        let todos = repo.get_all().await;
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0], todo);
    }

    #[tokio::test]
    async fn test_delete_todo() {
        let repo = MemoryTodoRepository::new();
        let todo = Todo::new(1, "description 1".to_string(), false);
        repo.create_todo(&todo)
            .await
            .expect("Expected to create todo successfully");
        repo.delete_todo(&todo.id)
            .await
            .expect("Expected to delete todo successfully");
        let todos = repo.get_all().await;
        assert_eq!(todos.len(), 0);
    }

    #[tokio::test]
    async fn test_update_todo() {
        let repo = MemoryTodoRepository::new();
        let todo = Todo::new(1, "description 1".to_string(), false);
        repo.create_todo(&todo).await.expect("");
        let mut updated = todo.clone();
        updated.set_description("updated".to_string());
        repo.update_todo(&updated).await.expect("");
        let todos = repo.get_all().await;
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0], updated);
    }
}
