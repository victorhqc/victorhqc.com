pub mod image_size;
pub mod photo;

use aws_sdk_s3::Client;
pub use aws_sdk_s3::primitives::ByteStream;

#[derive(Clone)]
pub struct S3 {
    client: Client,
    bucket_name: String,
}

impl S3 {
    pub async fn new(bucket_name: &str) -> Self {
        let sdk_config = aws_config::load_from_env().await;
        let s3_config = aws_sdk_s3::config::Builder::from(&sdk_config)
            .force_path_style(true)
            .build();

        S3 {
            client: Client::from_conf(s3_config),
            bucket_name: bucket_name.to_string(),
        }
    }
}
