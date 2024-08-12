FROM docker.io/library/rust:1-slim AS toolchain
ENV RUSTUP_TOOLCHAIN=nightly
COPY ./rust-toolchain.toml .
RUN cargo install trunk --locked


FROM toolchain AS build
WORKDIR /app
COPY . .
RUN trunk build --release --locked


FROM docker.io/library/caddy:2-alpine
EXPOSE 2015
WORKDIR /app

USER root:root
COPY --from=build /app/dist /app

USER 165512:0
CMD ["caddy", "file-server", "--listen", ":2015"]
