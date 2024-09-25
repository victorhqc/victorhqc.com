#!/bin/bash

export RUST_LOG=api=debug
export RUST_DEBUG=1

cargo watch -x run -i schema.gql
