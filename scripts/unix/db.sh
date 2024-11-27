#!/bin/sh

export RUST_LOG=api=debug,sqlx::query=debug,rocket=info
export RUST_DEBUG=1

if [[ "$1" == "--production" ]]; then
  echo "Production DB"
  export DATABASE_URL=sqlite:com.victorhqc.db
else
  echo "Development DB"
  export DATABASE_URL=sqlite:development.db
fi



sqlx database create
sqlx db setup --source=core/migrations
cargo sqlx prepare --workspace
cargo sqlx prepare --check --workspace
