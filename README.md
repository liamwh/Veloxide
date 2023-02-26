<!-- markdownlint-disable MD033 -->
# Velox: A template for building web APIs in Rust

---
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
            <img src="https://img.shields.io/badge/License-MIT-blue.svg)">
        </a>
    </p>

---

The key features are:

- **Highly Performant**: Velox is built on top of the [Tokio](https://tokio.rs) async runtime and [Axum framework](https://github.com/tokio-rs/axum), which leverage the power of Rust's async/await syntax and zero-cost abstractions to give bare-metal performance.
- **Fast to code**: Increase the speed of development by being simple, flexible and easy to use. Rust naturally pushes bugs left to the compiler, so less time is spent debugging code, and more time is spent delivering value.
- **Fewer bugs**: All components of Velox are written in [Rust](https://www.rust-lang.org), which is known for its safety and reliability.
- **Standards-based**: Based on (and fully compatible with) the open standards for APIs: [OpenAPI](https://github.com/OAI/OpenAPI-Specification) and [JSON Schema](https://json-schema.org/specification.html).
- **Cloud Native**: Velox comes pre-configured with [OpenTelemetry](https://opentelemetry.io/) for distributed tracing and metrics ready for collection by [Prometheus](https://prometheus.io/).

## Design Patterns

Velox implements the following design patterns to support maintainability and flexibility:

- **[Layered Architecture](https://en.wikipedia.org/wiki/Multitier_architecture)**: The codebased is divided into layers, each with a specific responsibility, as per the principles of [Domain-Driven Design](https://en.wikipedia.org/wiki/Domain-driven_design). This makes the application easier to understand and maintain.
- **[Dependency Injection](https://en.wikipedia.org/wiki/Dependency_injection)**: The application comes pre-configured with dependency injection to make subsituting dependencies, such as the persistence layer, easier.
- **[CQS](https://en.wikipedia.org/wiki/Command%E2%80%93query_separation)**: The application uses the CQS pattern to separate the read and write models. *CQRS coming soon!

## What's included?

| Feature | Crate(s) of Significance |
| --- | --- |
| Web Server | [Axum](https://docs.rs/axum/latest/axum/) |
| OpenAPI Doc Generation | [Utopia](https://docs.rs/utoipa/latest/utoipa/) |
| Async Runtime | [Tokio](https://docs.rs/tokio/latest/tokio/index.html) |
| Tracing | [Tracing](https://docs.rs/tracing/latest/tracing/) & [Tracing OpenTelemetry](https://docs.rs/tracing-otracing_opentelemetry/) & [OpenTelemetry-Jaeger](https://docs.rs/opentelemetry-jaeger/latest/opentelemetry_jaeger/) |
| Metrics | [Axum Prometheus](https://docs.rs/axum-prometheus/latest/axum_prometheus/) |
| Serializing & Deserializing | [Serde](https://docs.rs/serde/latest/serde/index.html) ([yaml](https://docs.rs/serde_yaml/latest/serde_yaml/) & [json](https://docs.rs/serde_json/latest/serde_json/)) |
| Async Database Driver (SQL)* | [Sqlx](https://docs.rs/sqlx/latest/sqlx/) |
| Async Object Relational Mapping* | [SeaORM](https://docs.rs/sea-orm/latest/sea_orm/) |
| Mocking | [mockall](https://docs.rs/mockall/latest/mockall/) |
| Error Handling | [thiserror](https://docs.rs/thiserror/latest/thiserror/) |
| Behavior Driven Development / Cucumber Testing | [Cucumber](https://docs.rs/cucumber/latest/cucumber/) |
| Loading env variables & .env file | [Dotenvy](https://docs.rs/dotenvy/latest/dotenvy/) |
| Improved assertion difference identification | [Pretty Assertions](https://docs.rs/pretty_assertions/latest/pretty_assertions/) |

***Note:** Investigation into selecting the preferred crate (sqlx or SeaORM) for interacting with the database is ongoing.

## Getting started

1. Clone this repo
1. (Optional): Run the below command to start the supporting containers.

    ```bash
    docker-compose up -d
    ```

1. Run the below command to start the application.

    ```bash
    cargo run
    ```

1. Browse to `http://localhost:4005/swagger-ui/` to see the interactive documentation.
1. To use the login security restricted endpoints, the api key is `example_api_key`.

### Supporting containers

- **Jaeger**: Traces will be sent to Jaeger, which can be accessed at `http://localhost:16686`.
- **Grafana**: Will be available at `http://localhost:3000`. The default username and password are both `admin`. Prometheus is already configured as the default data source.
- **Prometheus** will be available at `http://localhost:9090`.
- **Postgres** will be be listening on port `5432` for new connections.

## Why the name?

Velox is latin for "swift", "rapid" or "quick". Just like this stack 😉

## Roadmap to v1

### In-progress

- [ ] Make todo done field an enum instead of bool for demonstrative purposes
- [ ] Add a project model with a has-many relationship to the todo model
- [ ] Consider adding [SeaORM](https://www.sea-ql.org/SeaORM) for async Object Relational Mapping instead of writing raw SQL

### Endpoints

- [ ] Consider using [born](https://docs.rs/born/latest/born/) for struct generation to implement different read and write models
- [ ] Clean up error handling and propagation from domain layer to presentation layer
- [ ] Don't post a TODO ID in request body, get a UUID assigned
- [ ] Include functionality to generically search by any property via the API

### Testing

- [ ] Once the models and persistence of entities is finalised, add further testing of the infrastructure and presentation layers

### Other

- [ ] Add "Mark to-do as done" endpoint with tonic (gRPC)
- [ ] Implement CQRS
- [ ] Add a clear aggregate root, entity and value object example.
- [ ] Create a cool logo!
- [ ] Use CFG to enable/disable features (e.g. database selection, tracing, metrics)
- [ ] If we keep SeaORM, consider [Seaography](https://www.sea-ql.org/Seaography/) for GraphQL

### Done

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
