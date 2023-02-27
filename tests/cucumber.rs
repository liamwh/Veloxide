use cucumber::{given, then, when, World};
use velox_todo_api::domain::todo::*;

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct TodoWorld {
    todo: Todo,
}

#[given("a todo item")]
async fn a_todo_item(world: &mut TodoWorld) {
    world.todo = Todo::new(1, "Get started building my new API!".to_string(), false);
}

#[when("I mark it as completed")]
async fn mark_it_as_completed(world: &mut TodoWorld) {
    world.todo.set_status(true);
}

#[then("it should be completed")]
async fn it_should_be_completed(world: &mut TodoWorld) {
    assert!(world.todo.done);
}

// This runs before everything else, things can be setup here.
#[tokio::main]
async fn main() {
    TodoWorld::run("tests/features").await;
}
