# victorhqc.com

## How it works

This website hosts my basic information as well as my photography portfolio.
For this 1st iteration the website is pretty simple. It consists of a FE using
Next.js and a BE using Rocket.rs, as for the photos, those are being manually
uploaded to an S3 Bucket and recording its location in a SQLite DB.

Could this be done differently? Most likely, but I don't really want to
over-complicate it for now. Even if the deployment needs to upload the binary
and  SQLite DB to the server.

An additional consideration is to avoid S3 fees from Amazon. It's virtually
free to upload to a bucket, but every time data is transferred from the bucket
to any client it costs money. One way to minimize this cost, is to fetch the
images on boot and save them in memory. The bootstrap will be slow, but it will
keep the cost low.

In a future iteration I plan to have a simple backoffice to handle the S3 upload
and photos management, but that's a maybe and definitely in the future.

## Requirements

- Rust >= 1.78.0
- sqlx-cli:

  ```sh
  cargo install sqlx-cli
  cargo install cargo-watch
  ```
  
## CLI Backoffice

Since this Site won't have a UI or any kind of management outside my computer,
all the CRUD operations will happen locally, and I'll just ship the DB on every
release, I have to have someway of managing it, so a simple CLI will do.

This CLI reads the EXIF information using the trusty exiftool, stores it in the
DB and uploads the file to an S3 Bucket.

## API Development

Make sure you're in the `api` path first.

```sh
cd api
```

Prepare the environment variables by creating an `.env` file

```sh
cp .env.example .env
```

Create an initial DB

```sh
sqlx db create
sqlx db setup
```

Run the project

```sh
cargo run

# To recompile on change
cargo watch -x run
```

## Database

Add a new migration

```sh
sqlx migrate add -r <name>
```

Run  migrations again
```sh
sqlx migrate run
```
