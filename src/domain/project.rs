#[cfg(test)]
use mockall::{automock, predicate::*};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use super::todo::*;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, FromRow, PartialEq)]

pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub todos: Vec<Todo>,
}

#[cfg_attr(test, automock)]
impl Project {
    pub fn new(id: i32, name: String, description: String, todos: Vec<Todo>) -> Self {
        Self {
            id,
            name,
            description,
            todos,
        }
    }

    pub fn add_todo(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    pub fn remove_todo_by_id(&mut self, id: i32) {
        self.todos.retain(|todo| todo.id != id);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn project_new() {
        let project = Project::new(
            1,
            "My first project".to_string(),
            "My first project description".to_string(),
            vec![],
        );
        assert_eq!(project.id, 1);
        assert_eq!(project.name, "My first project");
        assert_eq!(project.description, "My first project description");
        assert_eq!(project.todos.len(), 0);
    }

    #[test]
    fn add_todo_with_empty_todo_list() {
        let mut project = Project::new(
            1,
            "My first project".to_string(),
            "My first project description".to_string(),
            vec![],
        );
        let todo = Todo::new(1, "Get started building my new API!".to_string(), false);
        project.add_todo(todo.clone());
        assert_eq!(
            project.todos.into_iter().next().expect("Expected a todo"),
            todo
        );
    }

    #[test]
    fn add_todo_with_non_empty_todo_list() {
        let mut project = Project::new(
            1,
            "My first project".to_string(),
            "My first project description".to_string(),
            vec![],
        );
        let todo = Todo::new(1, "Get started building my new API!".to_string(), false);
        project.add_todo(todo);
        let todo = Todo::new(
            2,
            "Get started building my new API! (updated)".to_string(),
            false,
        );
        project.add_todo(todo);
        assert_eq!(project.todos.len(), 2);
    }

    #[test]
    fn remove_todo_by_id() {
        let mut project = Project::new(
            1,
            "My first project".to_string(),
            "My first project description".to_string(),
            vec![],
        );
        let todo = Todo::new(1, "Get started building my new API!".to_string(), false);
        project.add_todo(todo);
        let todo2 = Todo::new(
            2,
            "Get started building my new API! (updated)".to_string(),
            false,
        );
        project.add_todo(todo2.clone());
        project.remove_todo_by_id(1);
        assert_eq!(
            project.todos.into_iter().next().expect("Expected a todo"),
            todo2
        );
    }
}
