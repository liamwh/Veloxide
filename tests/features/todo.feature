Feature: Todo feature

  Scenario: If I mark a todo as completed it should be completed
    Given a todo item
    When I mark it as completed
    Then it should be completed

 Scenario: I want to be able to give a table of todos in and get a table of todos out
    Given the following todos
      | id | Description                      | Completed |
      | 1  | Get started building my new API! | false     |
      | 2  | Get started building my new API! | false     |
      | 3  | Get started building my new API! | true     |
    When I mark them all as completed
    Then I expect the following todos
      | completed | id | Description                      |
      | true      | 1  | Get started building my new API! |
      | true      | 2  | Get started building my new API! |
      | false     | 3  | Get started building my new API! |
