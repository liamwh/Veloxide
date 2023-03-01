Feature: Todo feature

  Scenario: If I mark a todo as completed it should be completed
    Given a todo item
    When I mark it as completed
    Then it should be completed

 Scenario: I want to be able to give a table of todos in and get a table of todos out
    Given the following todos
      | ID | Description                           | Completed |
      | 1  | Get started building my new API!      | false     |
      | 2  | Seriously, why haven't I started yet? | true      |
      | 3  | Rust makes it too easy!               | false     |
    When I mark them all as completed
    Then I expect the following todos
      | ID | Description                           | Completed |
      | 1  | Get started building my new API!      | true      |
      | 2  | Seriously, why haven't I started yet? | true      |
      | 3  | Rust makes it too easy!               | true      |


 Scenario: I want to be able to give a table of todos in and get a table of todos out, and supply the columns in whatever order I want
    Given the following todos
      | Completed | ID | Description                           |
      | false     | 1  | Get started building my new API!      |
      | true      | 2  | Seriously, why haven't I started yet? |
      | false     | 3  | Rust makes it too easy!               |
    When I mark them all as completed
    Then I expect the following todos
      | ID | Description                           | Completed |
      | 1  | Get started building my new API!      | true      |
      | 2  | Seriously, why haven't I started yet? | true      |
      | 3  | Rust makes it too easy!               | true      |