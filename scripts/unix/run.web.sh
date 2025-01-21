#!/bin/sh
export RUST_LOG="api_victorhqc_com=error,web_victorhqc_com=error,core_victorhqc_com=error,sqlx::query=error"

export WEB_PORT=9998
export WEB_API_HOST=http://localhost:9999
export WEB_ROOT=web/
export DATABASE_URL="sqlite:web/analytics.db"

./target/release/web-victorhqc-com
