#!/bin/sh

export RUST_LOG=api=debug,sqlx::query=debug,rocket=info
export RUST_DEBUG=1
export DATABASE_URL=sqlite:development.db

cargo watch -i schema.gql -x 'run -p api-victorhqc-com'
