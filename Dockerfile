FROM node:20-alpine AS web_builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM rust:1.82-alpine AS rust_builder
WORKDIR /app
COPY rust-server ./rust-server
RUN cargo build --release --manifest-path rust-server/Cargo.toml

FROM alpine:3.20
WORKDIR /app
RUN adduser -D appuser
COPY --from=rust_builder /app/rust-server/target/release/astro_nomi_server /usr/local/bin/server
COPY --from=web_builder /app/dist ./dist
USER appuser
EXPOSE 8081
CMD ["server"]
