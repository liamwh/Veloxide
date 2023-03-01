use cucumber::{gherkin::Step, given, then, when, World};
use pretty_assertions::assert_eq;
use velox_todo_api::domain::todo::*;

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct TodoWorld {
    todo: Todo,
    todos: Vec<Todo>,
    expected_todos: Vec<Todo>,
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
    assert!(world.todo.completed);
}

// Table version:

#[derive(Debug, Clone, Default)]
pub struct TodoHeaderToColumnIndexMap {
    id: u16,
    description: u16,
    completed: u16,
}

async fn parse_todo_header_rows<'a>(
    world: &'a mut TodoWorld,
    step: &'a Step,
) -> (TodoHeaderToColumnIndexMap, &'a mut TodoWorld, &'a Step) {
    let mut map_header_to_column_index = TodoHeaderToColumnIndexMap::default();
    if let Some(table) = step.table.as_ref() {
        // Iterate through the header row and assign the column names a hash map
        for (index, header) in table.rows[0].iter().enumerate() {
            match header.to_lowercase().as_str() {
                "id" => map_header_to_column_index.id = index as u16,
                "description" => map_header_to_column_index.description = index as u16,
                "completed" => map_header_to_column_index.completed = index as u16,
                _ => panic!("Unexpected header in table"),
            }
        }
    }
    return (map_header_to_column_index, world, step);
}

async fn parse_todo_from_row<'a>(
    map_header_to_column_index: &TodoHeaderToColumnIndexMap,
    row: &Vec<String>,
    world: &'a mut TodoWorld,
) -> (Todo, &'a mut TodoWorld) {
    let id = row[map_header_to_column_index.id as usize]
        .parse::<i32>()
        .expect("Expected a 'id' value in this column, but got something else");
    let description = row[map_header_to_column_index.description as usize].to_string();
    let completed = row[map_header_to_column_index.completed as usize]
        .parse::<bool>()
        .expect("Expected a 'completed' value in this column, but got something else");
    return (Todo::new(id, description, completed), world);
}

#[given("the following todos")]
async fn the_following_todos(world: &mut TodoWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        let (map_header_to_column_index, world, _step) = parse_todo_header_rows(world, step).await;

        for row in table.rows.iter().skip(1) {
            // NOTE: skip header row
            let id = row[map_header_to_column_index.id as usize]
                .parse::<i32>()
                .expect("Expected a 'id' value in this column, but got something else");
            let description = row[map_header_to_column_index.description as usize].to_string();
            let completed = row[map_header_to_column_index.completed as usize]
                .parse::<bool>()
                .expect("Expected a 'completed' value in this column, but got something else");
            world.todos.push(Todo::new(id, description, completed));
        }
    }
}

#[when("I mark them all as completed")]
async fn mark_them_all_as_completed(world: &mut TodoWorld) {
    for todo in world.todos.iter_mut() {
        todo.set_status(true);
    }
}

#[then("I expect the following todos")]
async fn i_expect_the_following_todos(world: &mut TodoWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        let (map_header_to_column_index, world, _step) = parse_todo_header_rows(world, step).await;

        for row in table.rows.iter().skip(1) {
            // NOTE: skip header row
            let (expected_todo, world) =
                parse_todo_from_row(&map_header_to_column_index, row, world).await;
            world.expected_todos.push(expected_todo);
        }
    }

    for (index, todo) in world.todos.iter().enumerate() {
        assert_eq!(
            todo.id, world.expected_todos[index].id,
            "expected id: {} but got: {}",
            world.expected_todos[index].id, todo.id
        );
        assert_eq!(
            todo.description, world.expected_todos[index].description,
            "expected description: {} but got: {}",
            world.expected_todos[index].description, todo.description
        );
        assert_eq!(
            todo.completed, world.expected_todos[index].completed,
            "expected completed: {} but got: {}",
            world.expected_todos[index].completed, todo.completed
        );
    }
}

// This runs before everything else, things can be setup here.
#[tokio::main]
async fn main() {
    TodoWorld::run("tests/features").await;
}
