# SeaORM Documentation

## How to add a new entity

1. Add a migration with the below CLI command

    ```bash
    sea-orm-cli migrate generate my_migration_name
    ```

1. Modify the migration file in /migration/src as desired.
1. Run the migration with the below command

    ```bash
    sea-orm-cli migrate
    ```

1. Generate the entity with the below command

    ```bash
    sea-orm-cli generate entity -o entity/src --with-serde both
    ```

1. Modify the entity file in /entity/src as desired.

Further reading:

- [Migrations](https://www.sea-ql.org/SeaORM/docs/migration/writing-migration/)
- [Generating Entities](https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/)
