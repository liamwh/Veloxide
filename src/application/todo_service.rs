use std::sync::Arc;

use async_trait::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};
use tracing::instrument;

use crate::domain::DynTodoRepo;
pub use crate::domain::*;
pub use crate::prelude::*;

#[async_trait]
#[cfg_attr(test, automock)]
pub trait TodoService {
    async fn get_all(&self) -> Vec<Todo>;
    async fn create_todo(&self, todo: &Todo) -> Result<Todo>;
    async fn delete_todo(&self, id: &i32) -> Result<()>;
    async fn get_todo_by_id(&self, id: &i32) -> Result<Todo>;
    async fn mark_todo_as_completed(&self, id: &i32) -> Result<Todo>;
}

pub type DynTodoService = Arc<dyn TodoService + Send + Sync>;

pub struct TodoServiceImpl {
    repository: DynTodoRepo,
}

impl TodoServiceImpl {
    pub fn new(repository: DynTodoRepo) -> Self {
        Self { repository }
    }
}

#[async_trait]
#[cfg_attr(test, automock)]
impl TodoService for TodoServiceImpl {
    #[instrument(skip(self))]
    async fn get_all(&self) -> Vec<Todo> {
        self.repository.get_all().await
    }

    #[instrument(skip(self))]
    async fn create_todo(&self, todo: &Todo) -> Result<Todo> {
        self.repository.create_todo(todo).await
    }

    #[instrument(skip(self))]
    async fn delete_todo(&self, id: &i32) -> Result<()> {
        self.repository.delete_todo(id).await
    }

    #[instrument(skip(self))]
    async fn get_todo_by_id(&self, id: &i32) -> Result<Todo> {
        self.repository.get_todo_by_id(id).await
    }

    #[instrument(skip(self))]
    async fn mark_todo_as_completed(&self, id: &i32) -> Result<Todo> {
        let mut todo = self.repository.get_todo_by_id(id).await?;
        todo.set_status(true);
        self.repository.update_todo(&todo).await
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn todo_service_get_by_id() {
        // Arrange
        let mut todo_repository = MockTodoRepository::new();
        todo_repository
            .expect_get_todo_by_id()
            .with(eq(1))
            .times(1)
            .returning(|_| {
                Box::pin(async {
                    Ok(Todo {
                        id: 1,
                        description: "Test".to_string(),
                        done: false,
                    })
                })
            });
        let todo_service = TodoServiceImpl::new(Arc::new(todo_repository));

        // Act
        let todo = todo_service.get_todo_by_id(&1).await.unwrap();

        // Assert
        assert_eq!(todo.id, 1);
        assert_eq!(todo.description, "Test");
        assert_eq!(todo.done, false);
    }

    #[tokio::test]
    async fn mark_todo_as_completed() {
        // Arrange
        let mut todo_repository = MockTodoRepository::new();
        todo_repository
            .expect_get_todo_by_id()
            .with(eq(1))
            .times(1)
            .returning(|_| {
                Box::pin(async {
                    Ok(Todo {
                        id: 1,
                        description: "Test".to_string(),
                        done: false,
                    })
                })
            });

        todo_repository
            .expect_update_todo()
            .with(eq(Todo {
                id: 1,
                description: "Test".to_string(),
                done: true,
            }))
            .times(1)
            .returning(|_| {
                Box::pin(async {
                    Ok(Todo {
                        id: 1,
                        description: "Test".to_string(),
                        done: true,
                    })
                })
            });

        let todo_service = TodoServiceImpl::new(Arc::new(todo_repository));

        // Act
        let todo = todo_service
            .mark_todo_as_completed(&1)
            .await
            .expect("Expected mark todo as completed to be ok");

        // Assert
        assert_eq!(todo.id, 1);
        assert_eq!(todo.description, "Test");
        assert_eq!(todo.done, true);
    }

    #[tokio::test]
    async fn create_todo() {
        // Arrange
        let mut todo_repository = MockTodoRepository::new();
        todo_repository
            .expect_create_todo()
            .with(eq(Todo {
                id: 1,
                description: "Test".to_string(),
                done: false,
            }))
            .times(1)
            .returning(|_| {
                Box::pin(async {
                    Ok(Todo {
                        id: 1,
                        description: "Test".to_string(),
                        done: false,
                    })
                })
            });

        let todo_service = TodoServiceImpl::new(Arc::new(todo_repository));

        // Act
        let todo = todo_service
            .create_todo(&Todo {
                id: 1,
                description: "Test".to_string(),
                done: false,
            })
            .await
            .expect("Expected create todo to be ok");

        // Assert
        assert_eq!(todo.id, 1);
        assert_eq!(todo.description, "Test");
        assert_eq!(todo.done, false);
    }

    #[tokio::test]
    async fn delete_todo() {
        // Arrange
        let mut todo_repository = MockTodoRepository::new();
        todo_repository
            .expect_delete_todo()
            .with(eq(1))
            .times(1)
            .returning(|_| Box::pin(async { Ok(()) }));

        let todo_service = TodoServiceImpl::new(Arc::new(todo_repository));

        // Act
        let result = todo_service.delete_todo(&1).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_all() {
        // Arrange
        let mut todo_repository = MockTodoRepository::new();
        todo_repository.expect_get_all().times(1).returning(|| {
            Box::pin(async {
                vec![
                    Todo {
                        id: 1,
                        description: "Test".to_string(),
                        done: false,
                    },
                    Todo {
                        id: 2,
                        description: "Test 2".to_string(),
                        done: false,
                    },
                ]
            })
        });

        let todo_service = TodoServiceImpl::new(Arc::new(todo_repository));

        // Act
        let todos = todo_service.get_all().await;

        // Assert
        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].id, 1);
        assert_eq!(todos[0].description, "Test");
        assert_eq!(todos[0].done, false);
        assert_eq!(todos[1].id, 2);
        assert_eq!(todos[1].description, "Test 2");
        assert_eq!(todos[1].done, false);
    }
}
