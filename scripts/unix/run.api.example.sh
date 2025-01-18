#!/bin/sh

export RUST_LOG="api_victorhqc_com=error,cli_victorhqc_com=error,core_victorhqc_com=error,sqlx::query=error,rocket=error"
export DATABASE_URL="sqlite:/<PATH>/victorhqc.com/development.db"
export ROCKET_CACHED_PHOTO_TAGS="<TAG1>,<TAG2>"

export AWS_ACCESS_KEY_ID="<AWS_ACCESS_KEY_ID>"
export AWS_SECRET_ACCESS_KEY="<AWS_SECRET_ACCESS_KEY>"
export AWS_REGION="eu-central-1"
export AWS_BUCKET_NAME="<AWS_BUCKET_NAME>"

./target/release/api-victorhqc-com
