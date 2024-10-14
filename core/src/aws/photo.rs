use super::S3;
use crate::models::photo::Photo;
use aws_sdk_s3::{
    error::SdkError,
    operation::{
        get_object::{GetObjectError, GetObjectOutput},
        put_object::{PutObjectError, PutObjectOutput},
    },
    primitives::ByteStream,
};
use snafu::prelude::*;
use strum_macros::Display;

#[derive(Debug, Display)]
pub enum ImageSize {
    Hd,
    Md,
    Sm,
}

impl S3 {
    pub async fn upload_to_aws_s3(
        &self,
        data: (&Photo, ImageSize),
        buffer: Vec<u8>,
    ) -> Result<PutObjectOutput, Error> {
        let body = ByteStream::from(buffer);

        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(key(data))
            .body(body)
            .send()
            .await
            .context(UploadSnafu)
    }

    pub async fn download_from_aws_s3(
        &self,
        data: (&Photo, ImageSize),
    ) -> Result<GetObjectOutput, Error> {
        self.client
            .get_object()
            .bucket(&self.bucket_name)
            .key(key(data))
            .send()
            .await
            .context(DownloadSnafu)
    }
}

fn key((photo, size): (&Photo, ImageSize)) -> String {
    format!("{}_{}", photo.id, size)
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to upload file: {:?}", source))]
    Upload { source: SdkError<PutObjectError> },

    #[snafu(display("Failed to download file: {:?}", source))]
    Download { source: SdkError<GetObjectError> },
}
