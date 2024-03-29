# vim: set ft=dockerfile :
#Use cargo-chef to build dependencies separately from main binary
FROM lukemathwalker/cargo-chef:0.1.50-rust-1.65.0-bullseye AS chef

#Rocket requires the nightly Rust toolchain for now
RUN rustup default nightly

WORKDIR /builder

#Prepare a list of dependencies
FROM chef AS prepare

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build

COPY --from=prepare /builder/recipe.json recipe.json

#Build dependencies separately from main since dependencies take ages
RUN cargo chef cook --release --recipe-path recipe.json

COPY mfalib/src mfalib/src

COPY server/src server/src

RUN touch server/src/main.rs mfalib/src/lib.rs

RUN cargo build --release

#Finally, copy server over to a bare image to reduce bulk of final image
FROM debian:buster-slim AS runtime

EXPOSE 80

WORKDIR /app

COPY --from=build /builder/target/release/server .

#Rocket needs Rocket.toml config file in its runtime environment
COPY server/Rocket.toml .

ENTRYPOINT [ "/app/server" ]