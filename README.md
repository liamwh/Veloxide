# Velox: A template for building web APIs in Rust
<!-- markdownlint-disable MD033 -->
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
            <img src="https://img.shields.io/badge/License-MIT-blue.svg">
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
- **[Dependency Injection](https://en.wikipedia.org/wiki/Dependency_injection)**: The application comes pre-configured with dependency injection to make subsituting dependencies, such as the database, easier.
- **[CQS](https://en.wikipedia.org/wiki/Command%E2%80%93query_separation)**: The application uses the CQS pattern to separate the read and write models. *CQRS coming soon!

Note: Additional documentation on the design and implementation of Velox be found in the [docs](https://github.com/liamwh/velox/tree/main/docs) folder.

## What's included?

| Feature | Crate(s) of Significance |
| --- | --- |
| Web Server | [Axum](https://docs.rs/axum/latest/axum/) |
| OpenAPI Doc Generation | [Utopia](https://docs.rs/utoipa/latest/utoipa/) |
| Async Runtime | [Tokio](https://docs.rs/tokio/latest/tokio/index.html) |
| Tracing | [Tracing](https://docs.rs/tracing/latest/tracing/) & [Tracing OpenTelemetry](https://docs.rs/tracing-opentelemetry/latest/) & [OpenTelemetry-Jaeger](https://docs.rs/opentelemetry-jaeger/latest/) & [Tracing Log](https://docs.rs/tracing-log/latest/tracing_log/index.html) |
| Metrics | [Axum Prometheus](https://docs.rs/axum-prometheus/latest/axum_prometheus/) |
| Serializing & Deserializing | [Serde](https://docs.rs/serde/latest/serde/index.html) ([yaml](https://docs.rs/serde_yaml/latest/serde_yaml/) & [json](https://docs.rs/serde_json/latest/serde_json/)) |
| Command Query Responsibility Segregation & Event Sourcing | [cqrs-es](https://docs.rs/cqrs-es/latest/cqrs_es/) |
| Async Database Driver (SQL)* | [SQLx](https://docs.rs/sqlx/latest/sqlx/) |
| Mocking | [mockall](https://docs.rs/mockall/latest/mockall/) |
| Error Handling | [thiserror](https://docs.rs/thiserror/latest/thiserror/) |
| Behavior Driven Development / Cucumber Testing | [Cucumber](https://docs.rs/cucumber/latest/cucumber/) |
| Loading env variables & .env file | [Dotenvy](https://docs.rs/dotenvy/latest/dotenvy/) |
| Improved assertion difference identification | [Pretty Assertions](https://docs.rs/pretty_assertions/latest/pretty_assertions/) |
| Supercharged derive attributes | [Derivative](https://mcarton.github.io/rust-derivative/latest/index.html)

### Supporting containers

Velox comes pre-configured with the following supporting containers found in the `docker-compose.yml` file:

- **Jaeger**: Traces will be sent to Jaeger, which can be accessed at `http://localhost:16686`.
- **Prometheus** will be available at `http://localhost:9090`.
- **Grafana**: Will be available at `http://localhost:3000`. The default username and password are both `admin`. Prometheus is already configured as the default data source.
- **Postgres** will be be listening on port `5432` for new connections. The connection string is loaded from the environment variable `DATABASE_URL`, which is pre-configured in the .env file.

Note that using the supporting containers is optional if you change the config-example.yaml to use the memory database instead of postgres.

## Getting started

1. First, make sure you have the required dependencies installed:

     - [Rust](https://www.rust-lang.org/tools/install)
     - [SQLx CLI](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli)
     - [Docker](https://docs.docker.com/get-docker/)
        - Required only if using the supporting containers

2. Clone this repo
3. (Optional): Run the below command to start the supporting containers.

    ```bash
    docker-compose up -d
    ```

4. Run the below command to create the database.

    ```bash
    sqlx migrate run
    ```

5. Run the below command to start the application.

    ```bash
    cargo run
    ```

6. Browse to `http://localhost:4005/swagger-ui/` to see the interactive documentation.
7. To use the login security restricted endpoints, the api key is `example_api_key`.

## Why the name?

Velox is latin for "swift", "rapid" or "quick". Just like this stack 😉

## Roadmap to v1

### In-progress

- [ ] Add [OPA](https://www.openpolicyagent.org/) + [Envoy](https://www.envoyproxy.io/) for authorization example
- [ ] Implement CQRS and the concepts of aggregates, entities, domain events and commands.
- [ ] Make todo "completed" field an enum instead of bool for demonstrative purposes
- [ ] Add a project model with a has-many relationship to the todo model

### Endpoints

- [ ] Consider using [born](https://docs.rs/born/latest/born/) for struct generation to implement different read and write models
- [ ] Clean up error handling and propagation from domain layer to presentation layer
- [ ] Don't post a TODO ID in request body, get a UUID assigned
- [ ] Include functionality to generically search by any property via the API

### Testing

- [ ] Once the models and persistence of entities is finalised, add further testing of the infrastructure and presentation layers
- [ ] Improve mocking of CQRS components with [mockall](https://docs.rs/mockall/latest/mockall/)

### Other

- [ ] Improve discoverability of the API using utopia
- [ ] Tidy up imports, use super::* where appropriate to reduce multiplication of imports
- [ ] Add [Tonic](https://docs.rs/tonic/latest/tonic/) (gRPC) example
- [ ] Use CFG to enable/disable features (e.g. database selection, tracing, metrics)
- [ ] Consider adding GraphQL in some form
- [ ] Create a cool logo!

### Done

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
