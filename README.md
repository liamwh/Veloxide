# Velox: A template for building web APIs in Rust

<!-- markdownlint-disable MD033 -->

<p align="center">
    <a href="https://github.com/liamwh/Velox"><img src="./docs/.assets/under-construction.jpg" alt="Velox"
            width=100%></a>
    <p align="center">
        <em>Velox: high performance, easy to learn, fast to code, ready for production*</em>
        <br>
        <em>*Production quality coming soon :)</em>
    </p>
    <p align="center">
        <a href="https://github.com/liamwh/velox/actions?query=workflow%3AVelox-ci+event%3Apush+branch%3Amain">
            <img src="https://github.com/liamwh/velox/workflows/velox-ci/badge.svg?event=push&branch=main">
        </a>
        <a href="https://codecov.io/gh/liamwh/velox">
            <img src="https://codecov.io/gh/liamwh/velox/branch/main/graph/badge.svg?token=CFVJ2XAMNL" />
        </a>
        <a href="https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html">
            <img src="https://img.shields.io/badge/rustc-1.65+-lightgray.svg"></a>
        <a href="https://github.com/rust-secure-code/safety-dance/">
            <img src="https://img.shields.io/badge/unsafe-forbidden-success.svg" />
        </a>
        <a href="https://github.com/liamwh/velox/blob/main/README.md">
            <img src="https://img.shields.io/badge/License-MIT-blue.svg">
        </a>
    </p>

---

The key features are:

- **Fast to code**: Velox increases the speed of development by being simple, flexible and easy to use. Rust naturally [shifts bugs left](https://en.wikipedia.org/wiki/Shift-left_testing) to the compiler, so less time is spent debugging code, and more time is spent delivering value.
- **Fewer bugs**: All components of Velox are written in [Rust](https://www.rust-lang.org), which is known for its safety and reliability [[1]](https://www.infoq.com/news/2021/04/rust-linux-kernel-development/) [[2]](https://security.googleblog.com/2023/01/supporting-use-of-rust-in-chromium.html) [[3]](https://security.googleblog.com/2022/12/memory-safe-languages-in-android-13.html)
- **Highly Performant**: Velox is built on top of the [Tokio](https://tokio.rs) async runtime and [Axum framework](https://github.com/tokio-rs/axum), which leverage the power of Rust's [async/await syntax](https://doc.rust-lang.org/reference/expressions/await-expr.html) and [zero-cost abstractions](https://doc.rust-lang.org/beta/embedded-book/static-guarantees/zero-cost-abstractions.html) to give blazingly fast bare-metal performance.
- **Standards-based**: Velox leverages the open standards for APIs: [OpenAPI](https://github.com/OAI/OpenAPI-Specification), [JSON Schema](https://json-schema.org/specification.html) and [GraphQL](https://graphql.org/). You choose how you want your API to be consumed.
- **Cloud Native**: Velox comes pre-configured with [OpenTelemetry](https://opentelemetry.io/) for distributed tracing and /metrics endpoint preconfigured for collection from [Prometheus](https://prometheus.io/).

## Design Patterns

Velox implements the following design patterns to support maintainability and flexibility:

- **[CQRS](https://martinfowler.com/bliki/CQRS.html)**: Velox uses Command Query Responsibility Segregation (CQRS) to help simplify and optimize the design by separating the read (view) and write (command) models.
- **[Event Sourcing](https://martinfowler.com/eaaDev/EventSourcing.html)**: Velox uses Event Sourcing to persist events to the database. Event sourcing provides a complete and accurate audit trail of changes made to a system, which can be useful for debugging, compliance, and various other purposes.
- **[Layered Architecture](https://en.wikipedia.org/wiki/Multitier_architecture)**: The codebase is divided into layers, each with a specific responsibility, as per the principles of [Domain-Driven Design](https://en.wikipedia.org/wiki/Domain-driven_design). This makes the application easier to understand and maintain.
- **[Dependency Injection](https://en.wikipedia.org/wiki/Dependency_injection)**: Velox comes pre-configured with dependency injection to make subsituting dependencies, such as the database, easier.

Further documentation on the design and implementation of Velox be found in the [/docs folder](https://github.com/liamwh/velox/tree/main/docs).

## What's included?

| Feature                                                   | Crate(s) of Significance                                                                                                                                                                                                                                                      | Notes                                                                                                                         |
| --------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Web Server                                                | [Axum](https://docs.rs/axum/latest/axum/)                                                                                                                                                                                                                                     | The endpoint path and timestamp metadata for each issued command are captured and stored in the database in the events table. |
| GraphQL | [async-graphql](https://docs.rs/async-graphql/latest/async_graphql/) |
| OpenAPI Doc Generation                                    | [Utopia](https://docs.rs/utoipa/latest/utoipa/)                                                                                                                                                                                                                               | Serves interactive documentation at `/swagger-ui`                                                                             |
| Async Runtime                                             | [Tokio](https://docs.rs/tokio/latest/tokio/index.html)                                                                                                                                                                                                                        |                                                                                                                               |
| Tracing                                                   | [Tracing](https://docs.rs/tracing/latest/tracing/) & [Tracing OpenTelemetry](https://docs.rs/tracing-opentelemetry/latest/) & [OpenTelemetry-Jaeger](https://docs.rs/opentelemetry-jaeger/latest/) & [Tracing Log](https://docs.rs/tracing-log/latest/tracing_log/index.html) | Use the [`#[instrument]`](https://docs.rs/tracing/latest/tracing/attr.instrument.html) macro to automatically generate new spans whenever a function is called! Also, all logs are automatically embedded in Trace spans by default! Has tracing ever been this easy?                                                                               |
| Metrics                                                   | [Axum Prometheus](https://docs.rs/axum-prometheus/latest/axum_prometheus/)                                                                                                                                                                                                    | Metrics are pre-configured for collection at /metrics                                                                         |
| Serializing & Deserializing                               | [Serde](https://docs.rs/serde/latest/serde/index.html) ([yaml](https://docs.rs/serde_yaml/latest/serde_yaml/) & [json](https://docs.rs/serde_json/latest/serde_json/))                                                                                                        |                                                                                                                               |
| Command Query Responsibility Segregation & Event Sourcing | [cqrs-es](https://docs.rs/cqrs-es/latest/cqrs_es/)                                                                                                                                                                                                                            |                                                                                                                               |
| Async Database Driver (SQL)                             | [SQLx](https://docs.rs/sqlx/latest/sqlx/)                                                                                                                                                                                                                                     | SQL queries are checked against the database for validity _at compile time_                                                   |
| Mocking                                                   | [mockall](https://docs.rs/mockall/latest/mockall/)                                                                                                                                                                                                                            | Leverage the power of Rust's macro system by using [`#[automock]`](https://docs.rs/mockall/latest/mockall/attr.automock.html) to automatically create mocks!                                                                                                                               |
| Error Handling                                            | [thiserror](https://docs.rs/thiserror/latest/thiserror/)                                                                                                                                                                                                                      |                                                                                                                               |
| Behavior Driven Development / Cucumber Testing            | [Cucumber](https://docs.rs/cucumber/latest/cucumber/)                                                                                                                                                                                                                         |                                                                                                                               |
| Loading env variables & .env file                         | [Dotenvy](https://docs.rs/dotenvy/latest/dotenvy/)                                                                                                                                                                                                                            |                                                                                                                               |
| Improved assertion difference identification              | [Pretty Assertions](https://docs.rs/pretty_assertions/latest/pretty_assertions/)                                                                                                                                                                                              | Highlights the difference in tests character by character                                                                     |
| Supercharged derive attributes                            | [Derivative](https://mcarton.github.io/rust-derivative/latest/index.html)                                                                                                                                                                                                     |

### Supporting containers

Velox comes pre-configured with the following supporting containers found in the `docker-compose.yml` file:

- **[Jaeger](https://www.jaegertracing.io)**: Traces will be sent to Jaeger, which can be accessed at `http://localhost:16686`.
- **[Prometheus](https://prometheus.io/)** will be available at `http://localhost:9090`.
- **[Grafana](https://grafana.com/)**: Will be available at `http://localhost:3000`. The default username and password are both `admin`. Prometheus is already configured as the default data source.
- **[Postgres](https://www.postgresql.org/)** will be be listening on port `5432` for new connections. The connection string is loaded from the environment variable `DATABASE_URL`, which is pre-configured in the .env file.
- **[Envoy](https://www.envoyproxy.io/)**: Coming soon.
- **[Open Policy Agent](https://www.openpolicyagent.org/)**: Coming soon.

Note that using the supporting containers is optional if you change the velox-config.yaml to use the memory database instead of postgres.

## Getting started

1. First, make sure you have the required dependencies installed:

   - [Rust](https://www.rust-lang.org/tools/install)
   - [SQLx CLI](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli)
   - [Docker](https://docs.docker.com/get-docker/)
     - Required only if using the supporting containers

2. Clone this repo
3. Run the below command to start the supporting containers:

   ```bash
   docker-compose up -d
   ```

4. Run the below command to run the database migrations:

   ```bash
   sqlx migrate run
   ```

5. Run the below command to start the application:

   ```bash
   cargo run
   ```

6. Browse to `http://localhost:4005/swagger-ui/` to see the interactive documentation.
7. To use the login security restricted endpoints, the api key is `example_api_key`.

## Why the name?

Velox is latin for "swift", "rapid" or "quick". Just like this stack 😉

## Roadmap to v1

### In-progress

- [ ] Expose the BankAccountView using GraphQL
- [ ] Add [OPA](https://www.openpolicyagent.org/) + [Envoy](https://www.envoyproxy.io/) for authorization example

### Domain

- [ ] Rip out todo model, leave BankAccount as the only aggregate
- [ ] Consider adding a second aggregate to demonstrate how to use multiple aggregates

### Testing

- [ ] Improve code coverage, leverage [mockall](https://docs.rs/mockall/latest/mockall/) where applicable

### Other

- [ ] Add [Tonic](https://docs.rs/tonic/latest/tonic/) ([gRPC](https://grpc.io/)) example
- [ ] Use Rust's built in feature functionality to enable/disable Velox features (e.g. tracing, metrics, graphql, gRPC etc)
- [ ] Create a Rust Book
- [ ] Create a cool logo!

### Done

- [x] Get a [GraphQL](https://docs.rs/async-graphql/latest/async_graphql/) server running
- [x] Implement CQRS and the concepts of aggregates, entities, domain events and commands.
- [x] Add Dependabot to the repo to keep dependencies up to date
- [x] Set up code coverage pipeline / badge on readme
- [x] Set up CI Pipeline
- [x] Load application configuration from YAML
- [x] Send traces to Jaeger
- [x] Expose metrics endpoint and scrape with Prometheus, visible in Grafana
- [x] Example test cases using mockall
- [x] Serve on port from environment variable
- [x] Configure OpenAPI Doc generation
- [x] At least one cucumber test for BDD testing example
- [x] Use DDD layers
- [x] Abstract repository, implment basic in-memory repository and postgres repository

## A quick note on dynamic dispatch

Velox uses dynamic dispatch to implement dependency injection. This is a trade-off between performance and flexibility. If you are building a high-performance application, you may want to consider using static dispatch (dependencies defined at compile time), or the [Enum Dispatch crate](https://docs.rs/enum_dispatch/latest/enum_dispatch/index.html) as a middleground solution.

## Additional Documentation

Can be found in the [docs](docs) folder.
