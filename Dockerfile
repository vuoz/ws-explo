# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/

# Want to help us make this template better? Share your feedback here: https://forms.gle/ybq9Krt8jtBL3iCk7

ARG RUST_VERSION=1.80.0-nightly
ARG APP_NAME=dasd

################################################################################
# Create a stage for building the application.

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git

FROM rustlang/rust:nightly AS prereq
#RUN cargo install cargo-binstall
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall cargo-chef -y
RUN cargo binstall cargo-leptos -y 
# target wasm32-unknown-unknown
RUN rustup target add wasm32-unknown-unknown
RUN curl -sL https://deb.nodesource.com/setup_20.x | bash 
RUN apt-get update && apt-get install nodejs
RUN npm install -g sass


FROM prereq AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM prereq as cacher
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --package=backend --bin=backend --target-dir=target/    --recipe-path recipe.json
RUN cargo chef cook --profile=wasm-release --package=frontend --target-dir=target/front --target=wasm32-unknown-unknown    --recipe-path recipe.json

FROM prereq as builder
COPY . /app
WORKDIR /app

# copy dependecies
COPY --from=cacher /app/target /app/target
COPY --from=cacher /usr/local/cargo /usr/local/cargo



# build the app with cargo leptos which also builds the js bindings



ENV SQLX_OFFLINE=true
COPY --from=planner /app/.sqlx /app/.sqlx
RUN cargo leptos build --release

FROM rustlang/rust:nightly as runner

WORKDIR /app
# copy app form builder
COPY --from=builder /app/target/release/backend /app/
COPY --from=builder /app/target/site /app/target/site
COPY --from=builder /app/views /app/views
COPY --from=planner /app/.docker.env /app/.env


# set env vars

ENV OUTPUT_NAME="ws-explo"
ENV LEPTOS_OUTPUT_NAME="ws-explo"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_ASSETS_DIR="assets"
ENV LEPTOS_SITE_ADDR="0.0.0.0:5000"


EXPOSE 5000


# start the application
CMD ["./backend"]
