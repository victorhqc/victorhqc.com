#!/bin/sh

export RUST_LOG=cli_victorhqc_com=debug,sqlx::query=debug,rocket=info
export RUST_DEBUG=1
export DATABASE_URL=sqlite:development.db

cargo run -p cli-victorhqc-com -- "$@"
