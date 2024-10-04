@echo off
setlocal
@REM   SET RUST_BACKTRACE=1
  SET RUST_DEBUG=1
  SET RUST_LOG=api=debug,sqlx::query=debug,rocket=info
  call cargo watch -i schema.gql -x run

endlocal
