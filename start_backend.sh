#!/bin/bash

# Run the SQLx migrations
echo RUNNING MIGRATIONS
cd /app/database 
sqlx migrate run

# Build the Rust project
echo BUILDING PROJECT
cd /app/backend 
cargo build --release

# Start your application
echo RUNNING APLICATION
cargo run --release

