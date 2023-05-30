# Implementation remarks

# Build

1. docker compose up
2. cargo install sqlx-cli
3. sqlx migrate run
4. cargo build

# Test

1. ./verify_all.sh

# Test with curl

## Payment api

### Create payment

```bash
curl -X POST http://127.0.0.1:4000/api/payments \
   -H 'Content-Type: application/json' \
   -d '{ "payment": { "amount": 1045, "card_number": "123451234512345" } }'
```

### Get payment

```bash
curl -X GET http://127.0.0.1:4000/api/payments/dfad3ca9-a18f-4a39-9bf6-dfdb96c2eceb
```

# kill server if socket busy

```bash
kill $(lsof -t -i:4000)
```

# Coverage

1. ```CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test```
2. ```grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/html```
3. ```open target/coverage/html/index.html```
