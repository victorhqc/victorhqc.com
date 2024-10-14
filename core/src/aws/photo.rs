use super::S3;
use crate::models::photo::Photo;
use aws_sdk_s3::{
    error::SdkError,
    operation::{
        get_object::{GetObjectError, GetObjectOutput},
        put_object::{PutObjectError, PutObjectOutput},
    },
    primitives::{ByteStream, ByteStreamError},
};
use snafu::prelude::*;
use std::path::Path;

impl S3 {
    pub async fn upload_to_aws_s3(
        &self,
        photo: &Photo,
        path: &Path,
    ) -> Result<PutObjectOutput, Error> {
        let body = ByteStream::from_path(path).await.context(FileSnafu)?;

        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(&photo.id)
            .body(body)
            .send()
            .await
            .context(UploadSnafu)
    }

    pub async fn download_from_aws_s3(&self, photo: &Photo) -> Result<GetObjectOutput, Error> {
        self.client
            .get_object()
            .bucket(&self.bucket_name)
            .key(&photo.id)
            .send()
            .await
            .context(DownloadSnafu)
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to get file: {:?}", source))]
    File { source: ByteStreamError },

    #[snafu(display("Failed to upload file: {:?}", source))]
    Upload { source: SdkError<PutObjectError> },

    #[snafu(display("Failed to download file: {:?}", source))]
    Download { source: SdkError<GetObjectError> },
}
