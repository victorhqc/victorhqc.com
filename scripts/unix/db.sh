#!/bin/bash

export RUST_LOG=api=debug,sqlx::query=debug,rocket=info
export RUST_DEBUG=1
export DATABASE_URL=sqlite:development.db

sqlx database create
sqlx db setup --source=core/migrations
cargo sqlx prepare --workspace
cargo sqlx prepare --check --workspace
