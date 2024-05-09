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

  ```bash
  cargo install sqlx-cli
  ```
