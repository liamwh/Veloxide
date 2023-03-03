use cucumber::{gherkin::Step, given, then, when, World};
use pretty_assertions::assert_eq;

mod cucumber_todo;

pub use cucumber_todo::TodoWorld;

// This runs before everything else, things can be setup here.
#[tokio::main]
async fn main() {
    TodoWorld::run("tests/features").await;
}
