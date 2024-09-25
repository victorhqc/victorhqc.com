@echo off
setlocal
@REM   SET RUST_BACKTRACE=1
  SET RUST_DEBUG=1
  SET RUST_LOG=api=debug
  call cargo watch -x run -i schema.gql

endlocal
