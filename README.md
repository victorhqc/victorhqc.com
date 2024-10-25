# victorhqc.com

# How it works

<img src="screenshots/architecture.png" height="300" />

This website hosts my basic information as well as my photography portfolio.
For this 1st iteration the website is pretty simple. It consists of a FE using
Next.js and a BE using Rocket.rs, as for the photos, those are being manually
uploaded to an S3 Bucket and recording its location in a SQLite DB.

Could this be done differently? Most likely, but I don't really want to
over-complicate it for now. Even if the deployment needs to upload the binary
and SQLite DB to the server.

An additional consideration is to avoid S3 fees from Amazon. It's virtually
free to upload to a bucket, but every time data is transferred from the bucket
to any client it costs money. One way to minimize this cost, is to fetch the
images on boot and save them in memory. The bootstrap will be slow, but it will
keep the cost low.

In a future iteration I plan to have a simple backoffice to handle the S3 upload
and photos management, but that's a maybe and definitely in the future.

# Deployment

<img src="./screenshots/deployment.png" height="500" />

The deployment is initiated by a GitHub release. The CI will take care of the
rest, which is:

1. Build binaries
2. Send binaries and DB to the sever
3. Restarts API in server
4. Trigger Vercel deployment

An important note is that the release **requires** the local DB to be attached,
this will in turn, be used by the API.

# Development

## Requirements

- Rust >= 1.78.0
- sqlx-cli:

  ```sh
  cargo install sqlx-cli
  cargo install cargo-watch -i schema.gql -x "run -p api-victorhqc-com"
  ```

Prepare the environment variables by creating an `.env` file

```sh
cp .env.example .env
cp .cargo/config.toml.example .cargo/config.toml 
```

The `.cargo/config.toml` requires an update, replace the `<ROOT_PATH>` with the
current path you have the project saved in.

```sh
pwd
```

In unix systems it should like

```
DATABASE_URL = "sqlite:/users/user/victorhqc.com/development.db"
```

In Windows it should look like

```
DATABASE_URL = "sqlite:C:\\Users\\user\\victorhqc.com\\development.db"
```

Create the initial DB

```sh
./scripts/unix/db.sh
```

For Windows

```bat
scripts\windows\db.bat
```

## CLI Backoffice

Since this Site won't have a UI or any kind of management outside my computer,
all the CRUD operations will happen locally, and I'll just ship the DB on every
release, I have to have someway of managing it, so a simple CLI will do.

This CLI reads the EXIF information using [exiftool](https://exiftool.org/),
then stores it in the DB and uploads the file to an S3 Bucket.

For this, make sure to run the exiftool installation

```sh
./scripts/unix/exiftool.sh
```

For Windows

```bat
scripts\windows\exiftool.bat
```

And to run the CLI

```sh
cargo run -p cli-victorhqc-com
cargo run -p cli-victorhqc-com -- --help
```

## API Development

Run the project

```sh
cargo run -p api-victorhqc-com
```

## Database

Add a new migration

```sh
sqlx migrate add -r <name>
```

Run migrations again

```sh
sqlx migrate run
```
