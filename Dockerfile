FROM rust:latest as planner
WORKDIR app

RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust:latest as cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:latest as builder
WORKDIR app
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin Rua

FROM rustbian:latest
WORKDIR app
COPY --from=builder /app/target/release/Rua /usr/local/bin
EXPOSE 3921
ENV SERVER_BIND=0.0.0.0:3921
ENTRYPOINT ["/usr/local/bin/Rua"]