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

FROM debian:buster-slim as rustbian
RUN apt-get install bash
RUN apt-get update
RUN apt-get install -y wget
RUN apt-get install -y build-essential
RUN apt-get install -y zlib1g-dev
ARG OPENSSL_VERSION=1.1.0g
RUN wget https://www.openssl.org/source/openssl-${OPENSSL_VERSION}.tar.gz
RUN tar xvfz openssl-${OPENSSL_VERSION}.tar.gz
RUN cd openssl-${OPENSSL_VERSION} && ./config && make && make install
RUN echo '/usr/local/lib' >> /etc/ld.so.conf
RUN cat /etc/ld.so.conf
RUN ldconfig
RUN echo 'export LD_LIBRARY_PATH=/usr/local/lib' >> ~/.bash_profile && . ~/.bash_profile
WORKDIR app
COPY --from=builder /app/target/release/Rua /usr/local/bin
EXPOSE 3921
ENV SERVER_BIND=0.0.0.0:3921
ENTRYPOINT ["/usr/local/bin/Rua"]



