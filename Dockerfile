ARG VERSION=1
FROM rust:${VERSION}-buster AS builder

############################
# STAGE 1 build the binary
############################

# The below line should match the package name in Cargo.toml
ARG APP_NAME="example-veloxide-api"

RUN cargo new --bin ${APP_NAME}

WORKDIR ./${APP_NAME}

COPY . ./

RUN cargo build --release

############################
# STAGE 2 build a small image
############################

FROM debian:buster-slim

ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Europe/Amsterdam \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /example-veloxide-api/target/release/example-veloxide-api ${APP}/example-veloxide-api

RUN chown -R $APP_USER:$APP_USER ${APP}

EXPOSE 8080

USER $APP_USER
WORKDIR ${APP}

# HEALTHCHECK --interval=5s --timeout=3s --start-period=5s --retries=3 CMD curl -f http://localhost:8080/health || exit 1

ENTRYPOINT ["./example-veloxide-api"]