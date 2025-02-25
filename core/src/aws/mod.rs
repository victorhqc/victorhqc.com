pub mod image_size;
pub mod photo;

pub use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;

#[derive(Clone)]
pub struct S3 {
    client: Client,
    bucket_name: String,
}

impl S3 {
    pub async fn new(bucket_name: &str) -> Self {
        let sdk_config = aws_config::load_from_env().await;

        S3 {
            client: Client::new(&sdk_config),
            bucket_name: bucket_name.to_string(),
        }
    }
}
