<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD041 -->

<p align="center">
    <a href="https://github.com/liamwh/Veloxide"><img src="./docs/.assets/Veloxide-Wordmark-FullBanner.jpg" alt="Veloxide"
            width=100%></a>
    <p align="center">
    </p>
    <p align="center">
        <a href="https://github.com/liamwh/veloxide/actions?query=workflow%3AVeloxide-ci+event%3Apush+branch%3Amain">
            <img src="https://github.com/liamwh/veloxide/workflows/veloxide-ci/badge.svg?event=push&branch=main">
        </a>
        <a href="https://codecov.io/gh/liamwh/veloxide">
            <img src="https://codecov.io/gh/liamwh/veloxide/branch/main/graph/badge.svg?token=CFVJ2XAMNL" />
        </a>
        <a href="https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html">
            <img src="https://img.shields.io/badge/rustc-1.65+-success.svg"></a>
        <a href="https://github.com/rust-secure-code/safety-dance/">
            <img src="https://img.shields.io/badge/unsafe-forbidden-success.svg" />
        </a>
        <a href="https://github.com/liamwh/veloxide/blob/main/README.md">
            <img src="https://img.shields.io/badge/License-MIT-success.svg">
        </a>
    </p>

---

Veloxide is a modern, high-performance, cloud-native web API template written in Rust.

The key features are:

- **Fast to code**: Veloxide increases the speed of development by being simple, flexible and easy to use. Rust naturally [shifts bugs left](https://en.wikipedia.org/wiki/Shift-left_testing) to the compiler, so less time is spent debugging code, and more time is spent delivering value.
- **Fewer bugs**: All components of Veloxide are written in [Rust](https://www.rust-lang.org), which is known for its safety and reliability [[1]](https://www.infoq.com/news/2021/04/rust-linux-kernel-development/) [[2]](https://security.googleblog.com/2023/01/supporting-use-of-rust-in-chromium.html) [[3]](https://security.googleblog.com/2022/12/memory-safe-languages-in-android-13.html)
- **Highly performant**: Veloxide is built on top of the [Tokio](https://tokio.rs) async runtime and [Axum framework](https://github.com/tokio-rs/axum), which leverage the power of Rust's [async/await syntax](https://doc.rust-lang.org/reference/expressions/await-expr.html) and [zero-cost abstractions](https://doc.rust-lang.org/beta/embedded-book/static-guarantees/zero-cost-abstractions.html) to give blazingly fast bare-metal performance.
- **Cloud native**: Veloxide comes pre-configured with [OpenTelemetry](https://opentelemetry.io/) for distributed tracing and a /metrics endpoint preconfigured for collection from [Prometheus](https://prometheus.io/).
- **Standards-based**: Veloxide leverages the open standards for APIs: [OpenAPI](https://github.com/OAI/OpenAPI-Specification), [JSON Schema](https://json-schema.org/specification.html) and [GraphQL](https://graphql.org/). You choose how you want your API to be consumed.

## Design Patterns

Veloxide implements the following design patterns to support maintainability and flexibility:

- **[CQRS](https://learn.microsoft.com/en-us/azure/architecture/patterns/cqrs)**: Veloxide uses Command Query Responsibility Segregation (CQRS) to help simplify and optimize the design by separating the read (view) and write (command) models.
- **[Event Sourcing](https://martinfowler.com/eaaDev/EventSourcing.html)**: Veloxide uses Event Sourcing to persist domain events to the database. Event sourcing is used to tie the read and write models together, as well as providing a complete and accurate audit trail of changes made to a system, which can be useful for debugging, compliance, and various other purposes.
- **[Layered Architecture](https://en.wikipedia.org/wiki/Multitier_architecture)**: The codebase is divided into layers, each with a specific responsibility, as per the principles of [Domain-Driven Design](https://en.wikipedia.org/wiki/Domain-driven_design). This makes the application easier to understand and maintain.

Further documentation on the design and implementation of Veloxide be found in the [/docs folder](https://github.com/liamwh/veloxide/tree/main/docs).

## What's included?

### Key Components

| Component                                                   | Crate(s) of Significance                                                                                                                                                                                                                                                      | Notes                                                                                                                         |
| --------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Web Server                                                | [Axum](https://docs.rs/axum/latest/axum/), [Tower](https://docs.rs/tower/0.4.13/tower/)                                                                                                                                                                                                                                     | The endpoint path and timestamp metadata for each issued command are captured and stored in the database in the events table. |
| GraphQL | [async-graphql](https://docs.rs/async-graphql/latest/async_graphql/), [async-graphql-axum](https://docs.rs/async-graphql-axum/5.0.6/async_graphql_axum/)  |
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
| Code coverage generation | [cargo-llmvm-cov](https://github.com/taiki-e/cargo-llvm-cov) | |
| Automatic Typescript Binding Generation | [ts-rs](https://docs.rs/ts-rs/latest/ts_rs) | Automatically generates TypeScript interfaces for your Rust view models! |

### Supporting containers

Veloxide comes pre-configured with the following supporting containers found in the `docker-compose.yml` file:

- **[Jaeger](https://www.jaegertracing.io)**: Traces will be sent to Jaeger, which can be accessed at `http://localhost:16686`.
- **[Prometheus](https://prometheus.io/)** will be available at `http://localhost:9090`.
- **[Grafana](https://grafana.com/)**: Will be available at `http://localhost:3000`. The default username and password are both `admin`. Prometheus is already configured as the default data source.
- **[Postgres](https://www.postgresql.org/)** will be be listening on port `5432` for new connections. The connection string is loaded from the environment variable `DATABASE_URL`, which is pre-configured in the .env file.
- **[Envoy](https://www.envoyproxy.io/)**: Coming soon.
- **[Open Policy Agent](https://www.openpolicyagent.org/)**: Coming soon.

Note that using the supporting containers is optional if you change the veloxide-config.yaml to use the memory database instead of postgres.

## Getting started

Install the pre-requisites:

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)

Install the Veloxide CLI:

```sh
> cargo install veloxide
```

To create your own app:

```zsh
> veloxide init my-app

# Go to created folder
> cd my-app

# Install tools
> make tools.required

# Start the supporting containers, followed by the build process
> make dev

# Once done, open `my-app/` in your IDE

# Happy Coding!
```

- The OpenAPI interactive documentation is available at `http://localhost:8080/swagger-ui/`
- GraphQL Playground is available at `http://localhost:9000/`

## Why the name?

Velox + Oxide = Veloxide

- Velox: latin for "swift", "rapid" or "quick", just like development with this stack 😉
- Oxide: Rust is an iron oxide, and all components of Veloxide are written in Rust.

## Additional Documentation

Can be found in the [docs](docs) folder.

## Contributors

- **[Liam Woodleigh-Hardinge](https://bit.ly/422Y4Lo)**: Creator
- **[Lucas Moulton-Wotherspoon](https://bit.ly/3l5zqsS)**: Graphics and Design
- You? 😊
