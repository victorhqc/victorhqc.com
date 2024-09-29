#!/bin/bash

export RUST_LOG=api=debug,sqlx::query=debug
export RUST_DEBUG=1

cargo watch -x run -i schema.gql
