ARG VERSION=1
FROM rust:${VERSION}-buster AS builder

############################
# STAGE 1 build the binary
############################

# The below line should match the package name in Cargo.toml
ARG APP_NAME="example-veloxide-api"

RUN cargo new --bin ${APP_NAME}

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

EXPOSE 8080

ENV TZ=Europe/Amsterdam \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /${APP_NAME}/target/release/${APP_NAME} ${APP}/${APP_NAME}

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./${APP_NAME}"]