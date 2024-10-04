@echo off
setlocal
  SET RUST_DEBUG=1
  SET RUST_LOG=api=debug,sqlx::query=debug,rocket=info
  SET DATABASE_URL=sqlite:development.db
  call cargo watch -i schema.gql -x "run -p api"

endlocal
