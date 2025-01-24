# victorhqc.com

## How it works

<img src="screenshots/architecture.png" height="400" />

This website hosts my basic information as well as my photography portfolio. It
started as a simple idea and it evolved into a fairly complex application that
is in the over-engineered side. However, it was very satisfying to build, I
learned a few new things and I'm happy to see that is 90% Rust, although I had
to move the photo stack js code into a different repository to keep it JS from
taking over :)

<img src="screenshots/index.png" height="400" />

# Development

### Requirements

- Rust >= 1.78.0
- sqlx-cli:

  ```sh
  # The DB CLI
  cargo install sqlx-cli
  # For recompilation on code update while developing
  cargo install --locked bacon
  ```

Prepare the environment variables by creating an `.env` file

```sh
cp .env.example .env
cp .cargo/config.toml.example .cargo/config.toml 
```

The `.cargo/config.toml` requires an update, replace the needed keys for AWS.

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

# With hot reload
bacon
```

## Web Frontend Development

First, install the dependencies the website needs using the `web-dependencies.sh` script, just make sure `wget` is installed,
for Mac OSX do it with

```sh
brew install wget
```

And then run the script. It will download a copy of `tailwindcss` and `htmx`. We could use a CDN, but time has proven that
CDNs go down sometimes, and we want to avoid problems caused by 3rd parties as much as possible.

```sh
./scripts/unix/web-dependencies.sh
```


Make sure the API is running and then run the following

```sh
# With hot reload
bacon web
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

# Compilation

The service will run in a Linux machine, so targeting that platform is imperative.

## From Mac OSX

**Requirements**

1. Musl Target
  ```sh
  rustup target add x86_64-unknown-linux-musl
  ```
2. Musl Linker
  ```sh
  brew install FiloSottile/musl-cross/musl-cross
  ```

**Compilation**

```sh
cargo build --release --target x86_64-unknown-linux-musl
```

# Stress Testing API

To make sure the API runs smoothly, running stress tests is encouraged.

## Requirements

- [Drill](https://github.com/fcsonline/drill)

```sh
cargo install drill
```

Copy the script and benchmark file

```sh
cp scripts/unix/run.api.example.sh scripts/unix/run.api.sh
cp stress-tests/benchmark.example.yml stress-tests/benchmark.yml
cp stress-tests/benchmark.web.example.yml stress-tests/benchmark.web.yml
```

And replace the values in the script and benchmark file

## How to run

Make sure the API is running

```sh
cargo build --release
./scripts/unix/run.api.sh
./scripts/unix/run.web.sh
```

Then run the stress tests

```sh
drill --benchmark stress-tests/benchmark.yml --stats
drill --benchmark stress-tests/benchmark.web.yml --stats
```

# Deployment

The current deployment is pretty spartan. It's a basic automation where the
binaries and necessary files are shipped to the service through `scp` and then
some commands are manually run using `ssh`.

It requires that manual configuration is already in place. Meaning, having
[Nginx configured](https://www.digitalocean.com/community/tutorials/how-to-install-nginx-on-ubuntu-20-04#step-5-%E2%80%93-setting-up-server-blocks-(recommended))
as well as having the `systemd` services ready. There's a small description
on the configuration needed.

Once that is ready then the deployment script can be executed

```sh
# Compiling for release is mandatory to run before
cargo build --release --target x86_64-unknown-linux-musl

./scripts/unix/release.sh -k ~/.ssh/your-ssh-key -h victorhqc.com -u username -p path_in_server

# Or like this to install web dependencies
./scripts/unix/release.sh -k ~/.ssh/your-ssh-key -h victorhqc.com -u username -p path_in_server --install
```

## Requirements

- Nginx

### Domain

### Systemd

The services use `systemd` to manage restarts and configuration. Benjamin
Morel [has an excellent](https://medium.com/@benmorel/creating-a-linux-service-with-systemd-611b5c8b91d6) 
guide on how to set a service.

Once the services configured with the configuration stated below, one can
simply write

```sh
systemctl status api.victorhqc.com
systemctl restart api.victorhqc.com

systemctl status www.victorhqc.com
systemctl restart www.victorhqc.com
```

### API Configuration

The file for the API Service, I have it configured as

`/etc/systemd/system/api.victorhqc.com.env`
```
DATABASE_URL="<PATH>"
ROCKET_DATABASE_URL="<PATH>"
ROCKET_CACHED_PHOTO_TAGS="<COMMA_SEPARATED_TAGS>"
ROCKET_PORT=<PORT>

RUST_LOG = "api_victorhqc_com=error,core_victorhqc_com=error,sqlx::query=error,rocket=error"

AWS_ACCESS_KEY_ID=<AWS_ACCESS_KEY>
AWS_SECRET_ACCESS_KEY=<AWS_SECRET_ACCESS_KEY>
AWS_REGION=eu-central-1
AWS_BUCKET_NAME=<BUCKET_NAME>
```

`/etc/systemd/system/api.victorhqc.com.service`
```
[Unit]
Description=victorhqc.com API (api.victorhqc.com)
After=network.target
StartLimitIntervalSec=0

[Service]
Type=simple
Restart=always
RestartSec=1
User=<USERNAME>
ExecStart=<PATH>/linux-api-victorhqc-com
EnvironmentFile=/etc/systemd/system/api.victorhqc.com.env

[Install]
WantedBy=multi-user.target
```

### WEB Configuration

The config file for the web service is


`/etc/systemd/system/www.victorhqc.com.env`
```
WEB_PORT=<PORT>
WEB_API_HOST=https://api.victorhqc.com
WEB_ROOT=<PATH_TO_WEB_STATICS>/victorhqc.com/
DATABASE_URL=<PATH_TO_BINARY>/analytics.db
OUT_DIR=<PATH_TO_WEB_STATICS>/victorhqc.com/
REGEX_PATH=<PATH_TO_WEB_STATICS>/victorhqc.com/
```

`/etc/systemd/system/www.victorhqc.com.service`
```
After=network.target
StartLimitIntervalSec=0

[Service]
Type=simple
Restart=always
RestartSec=1
User=<USERNAME>
ExecStart=<PATH>/linux-web-victorhqc-com 
EnvironmentFile=/etc/systemd/system/www.victorhqc.com.env

[Install]
WantedBy=multi-user.target
```
