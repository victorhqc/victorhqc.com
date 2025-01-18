#!/bin/sh
export RUST_LOG="api_victorhqc_com=error,cli_victorhqc_com=error,core_victorhqc_com=error,sqlx::query=error,rocket=error"

export WEB_PORT=9998
export WEB_API_HOST=http://localhost:9999
export WEB_ROOT=web/


./target/release/web-victorhqc-com
