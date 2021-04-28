FROM debian:buster-slim


ENV RUST_LOG=info

COPY /app /app
WORKDIR /app

ENTRYPOINT ["/app/rho_runtime"]
