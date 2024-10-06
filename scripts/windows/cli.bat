@echo off
setlocal
  SET RUST_DEBUG=1
  SET RUST_LOG=cli_victorhqc_com=debug,sqlx::query=debug,rocket=info
  SET DATABASE_URL=sqlite:development.db
  call cargo run -p cli-victorhqc-com -- %*

endlocal
