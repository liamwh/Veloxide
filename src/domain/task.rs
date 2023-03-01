use super::*;
use async_trait::async_trait;
use cqrs_es::{Aggregate, DomainEvent, EventEnvelope, Query};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub struct TaskService;

#[derive(Debug, Deserialize)]
pub enum TaskCommand {
    CreateTodo { todo_id: i32, description: String },
    ChangeDescription { description: String },
    MarkAsComplete,
    MarkAsIncomplete,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskEvent {
    TaskCreated { todo_id: i32, description: String },
    DescriptionChanged { description: String },
    TaskMarkedAsComplete,
    TaskMarkedAsIncomplete,
}

impl DomainEvent for TaskEvent {
    fn event_type(&self) -> String {
        let event_type: &str = match self {
            TaskEvent::TaskCreated { .. } => "TodoCreated",
            TaskEvent::DescriptionChanged { .. } => "DescriptionChanged",
            TaskEvent::TaskMarkedAsComplete { .. } => "TodoMarkedAsComplete",
            TaskEvent::TaskMarkedAsIncomplete { .. } => "TodoMarkedAsIncomplete",
        };
        event_type.to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Debug, PartialEq)]
pub struct TaskError(String);

impl TaskError {
    pub fn new(message: &str) -> Self {
        TaskError(message.to_string())
    }
}

impl Display for TaskError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for TaskError {}

impl From<&str> for TaskError {
    fn from(message: &str) -> Self {
        TaskError(message.to_string())
    }
}

impl TaskService {}

struct SimpleLoggingQuery;

#[async_trait]
impl Query<Task> for SimpleLoggingQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<Task>]) {
        for event in events {
            println!("{}-{}\n{:#?}", aggregate_id, event.sequence, &event.payload);
        }
    }
}

#[derive(Serialize, Default, Deserialize, Debug, PartialEq)]
pub struct Task {
    pub task_id: i32,
    pub description: String,
    pub is_complete: bool,
}

#[async_trait]
impl Aggregate for Task {
    type Command = TaskCommand;
    type Event = TaskEvent;
    type Error = TaskError;
    type Services = TaskService;

    // This identifier should be unique to the system.
    fn aggregate_type() -> String {
        "Todo".to_string()
    }

    // The aggregate logic goes here. Note that this will be the _bulk_ of a CQRS system
    // so expect to use helper functions elsewhere to keep the code clean.
    #[instrument(skip(_services))]
    async fn handle(
        &self,
        command: Self::Command,
        _services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        // The handle method does not allow any mutation of the aggregate, state should be changed only by emitting events.
        match command {
            TaskCommand::CreateTodo {
                todo_id,
                description,
            } => {
                let event = TaskEvent::TaskCreated {
                    todo_id,
                    description,
                };
                Ok(vec![event])
            }
            TaskCommand::ChangeDescription { description } => {
                let event = TaskEvent::DescriptionChanged { description };
                Ok(vec![event])
            }
            TaskCommand::MarkAsComplete => {
                if self.is_complete {
                    return Err(TaskError::new("Task is already complete"));
                }
                let event = TaskEvent::TaskMarkedAsComplete;
                Ok(vec![event])
            }
            TaskCommand::MarkAsIncomplete => {
                if !self.is_complete {
                    return Err(TaskError::new("Task is already incomplete"));
                }
                let event = TaskEvent::TaskMarkedAsIncomplete;
                Ok(vec![event])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            TaskEvent::TaskCreated {
                todo_id,
                description,
            } => {
                self.task_id = todo_id;
                self.description = description;
            }
            TaskEvent::DescriptionChanged { description } => {
                self.description = description;
            }
            TaskEvent::TaskMarkedAsComplete => {
                self.is_complete = true;
            }
            TaskEvent::TaskMarkedAsIncomplete => {
                self.is_complete = false;
            }
        }
    }
}

#[cfg(test)]
mod aggregate_tests {
    use super::*;
    use cqrs_es::{mem_store::MemStore, test::TestFramework, CqrsFramework};

    type TaskTestFramework = TestFramework<Task>;

    #[test]
    fn test_create_task_publishes_event() {
        let expected = TaskEvent::TaskCreated {
            todo_id: 1,
            description: "Example description".to_string(),
        };

        TaskTestFramework::with(TaskService)
            .given_no_previous_events()
            .when(TaskCommand::CreateTodo {
                todo_id: 1,
                description: "Example description".to_string(),
            })
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_change_description_publishes_event() {
        let expected = TaskEvent::DescriptionChanged {
            description: "New description".to_string(),
        };

        TaskTestFramework::with(TaskService)
            .given(vec![TaskEvent::TaskCreated {
                todo_id: 1,
                description: "Example description".to_string(),
            }])
            .when(TaskCommand::ChangeDescription {
                description: "New description".to_string(),
            })
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_mark_as_complete_publishes_event() {
        let expected = TaskEvent::TaskMarkedAsComplete;

        TaskTestFramework::with(TaskService)
            .given(vec![TaskEvent::TaskCreated {
                todo_id: 1,
                description: "Example description".to_string(),
            }])
            .when(TaskCommand::MarkAsComplete)
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_mark_as_incomplete_publishes_event() {
        TaskTestFramework::with(TaskService)
            .given(vec![
                TaskEvent::TaskCreated {
                    todo_id: 1,
                    description: "Example description".to_string(),
                },
                TaskEvent::TaskMarkedAsComplete,
            ])
            .when(TaskCommand::MarkAsIncomplete)
            .then_expect_events(vec![TaskEvent::TaskMarkedAsIncomplete]);
    }

    #[test]
    fn test_mark_as_incomplete_returns_error_if_not_complete() {
        TaskTestFramework::with(TaskService)
            .given(vec![
                TaskEvent::TaskCreated {
                    todo_id: 1,
                    description: "Example description".to_string(),
                },
                TaskEvent::TaskMarkedAsIncomplete,
            ])
            .when(TaskCommand::MarkAsIncomplete)
            .then_expect_error(TaskError("Task is already incomplete".to_string()));
    }

    #[test]
    fn test_mark_as_complete_returns_error_if_already_complete() {
        TaskTestFramework::with(TaskService)
            .given(vec![
                TaskEvent::TaskCreated {
                    todo_id: 1,
                    description: "Example description".to_string(),
                },
                TaskEvent::TaskMarkedAsComplete,
            ])
            .when(TaskCommand::MarkAsComplete)
            .then_expect_error(TaskError("Task is already complete".to_string()));
    }

    #[tokio::test]
    async fn test_event_store() {
        let event_store = MemStore::<Task>::default();
        let query = SimpleLoggingQuery {};
        let cqrs = CqrsFramework::new(event_store, vec![Box::new(query)], TaskService);

        let aggregate_id = "aggregate-instance-A";

        // deposit $1000
        cqrs.execute(
            aggregate_id,
            TaskCommand::ChangeDescription {
                description: "new description".to_string(),
            },
        )
        .await
        .unwrap();

        // Mark it as complete
        cqrs.execute(aggregate_id, TaskCommand::MarkAsComplete)
            .await
            .unwrap();
    }
}
