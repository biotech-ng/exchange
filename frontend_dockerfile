# Initial image
FROM rust:1.68 AS builder

# Update, install, and configure
RUN set -ex \
    && apt update \
    && apt install -y build-essential \
    && cargo install wasm-pack \
    && rm -rf ${CARGO_HOME}/git/* \
    && rm -rf ${CARGO_HOME}/registry/* \
    && rm -rf /var/lib/apt/lists/* \
    && mkdir /app

# Setting up working dir for getting whole project
# Needed for compilation
WORKDIR /app
# Copy the exchange project 
COPY . .
# Set up the working dir
WORKDIR /app/slint_lib
# Build
RUN wasm-pack build --debug --target web

# Running image
FROM python:alpine3.17
# Copy the exchange project from builder
COPY --from=builder /app/slint_lib /app/slint_lib
# Set up the working dir
WORKDIR /app/slint_lib
# Hosting
CMD ["python3", "-m" , "http.server"]

