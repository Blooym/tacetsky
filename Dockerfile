###########
# Builder #
###########
FROM rust:alpine AS builder
WORKDIR /build

# Install build dependencies
RUN apk add --update build-base cmake libressl-dev

# Pre-cache dependencies
COPY ["Cargo.toml", "Cargo.lock", "./"]
RUN mkdir src \
    && echo "// Placeholder" > src/lib.rs \
    && cargo build --release \
    && rm src/lib.rs

# Build
ARG SQLX_OFFLINE true
COPY ./migrations ./migrations
COPY ./.sqlx ./.sqlx
COPY ["./src", "./src"]
RUN cargo build --release

###########
# Runtime #
###########
FROM alpine
RUN adduser -S -s /bin/false -D tacetsky
USER tacetsky
WORKDIR /opt/tacetsky
RUN mkdir /opt/tacetsky/data

ENV RUST_BACKTRACE=1
ENV DATABASE_URL=sqlite:///opt/tacetsky/data/db.sqlite3?mode=rwc
ENV DATA_PATH=/opt/tacetsky/data
COPY --from=builder /build/target/release/tacetsky /usr/local/bin/tacetsky
ENTRYPOINT ["/usr/local/bin/tacetsky", "start"]