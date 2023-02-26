Feature: Todo feature

  Scenario: If I mark a todo as completed it should be completed
    Given a todo item
    When I mark it as completed
    Then it should be completed