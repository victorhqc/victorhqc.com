@echo off
setlocal
  SET RUST_DEBUG=1
  SET RUST_LOG=api=debug,sqlx::query=debug,rocket=info
  SET DATABASE_URL=sqlite:development.db

  call sqlx database create
  call sqlx db setup --source=core/migrations
  call cargo sqlx prepare --workspace
  call cargo sqlx prepare --check --workspace

endlocal
