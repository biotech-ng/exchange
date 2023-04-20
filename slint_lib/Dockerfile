FROM rust:slim

LABEL "org.opencontainers.image.title"="Rust WASM"
LABEL "org.opencontainers.image.description"="Rust WASM Docker Image with cargo generate, trunk, yarn, git and deno pre-installed"
LABEL "org.opencontainers.image.authors"="SakaDream"

RUN set -ex \
    && apt-get update \
    && apt-get install -y curl ca-certificates pkg-config make git unzip --no-install-recommends \
    && curl -sL https://deb.nodesource.com/setup_16.x | bash - \
    && curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - \
    && echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list \
    && apt-get update \
    && apt-get install -y nodejs yarn --no-install-recommends \
    && rustup target add wasm32-unknown-unknown \
    && cargo install trunk \
    && curl -fsSL https://deno.land/x/install/install.sh | sh \
    && npm cache verify \
    && yarn cache clean --all \
    && rm -rf ${CARGO_HOME}/git/* \
    && rm -rf ${CARGO_HOME}/registry/* \
    && rm -rf /var/lib/apt/lists/* \
    && mkdir /app

# cargo-generate

WORKDIR /app

COPY . .

# 1. Doesn't work
RUN cargo install cargo-wasi
# RUN cargo wasi run 

# 2. Doesn't work
RUN rustup target add wasm32-wasi
RUN rustc build.rs --target wasm32-wasi

# 3. Doesn't work
# RUN wasm-pack --help
# RUN wasm-pack build --release --target web

RUN ls

