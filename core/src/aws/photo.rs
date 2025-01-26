use super::image_size::{ImageSize, ImageType};
use super::S3;
use crate::models::photo::Photo;
use aws_sdk_s3::{
    error::SdkError,
    operation::{
        delete_object::{DeleteObjectError, DeleteObjectOutput},
        get_object::{GetObjectError, GetObjectOutput},
        put_object::{PutObjectError, PutObjectOutput},
    },
};
use snafu::prelude::*;

pub use aws_sdk_s3::primitives::{ByteStream, ByteStreamError};

impl S3 {
    pub async fn upload_to_aws_s3(
        &self,
        data: (&Photo, &ImageSize, &ImageType),
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
        data: (&Photo, &ImageSize, &ImageType),
    ) -> Result<GetObjectOutput, Error> {
        self.client
            .get_object()
            .bucket(&self.bucket_name)
            .key(key(data))
            .send()
            .await
            .context(DownloadSnafu)
    }

    pub async fn remove_from_aws_s3(
        &self,
        data: (&Photo, &ImageSize, &ImageType),
    ) -> Result<DeleteObjectOutput, Error> {
        self.client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(key(data))
            .send()
            .await
            .context(RemoveSnafu)
    }
}

fn key((photo, size, photo_type): (&Photo, &ImageSize, &ImageType)) -> String {
    if photo_type == &ImageType::Webp {
        return format!("{}_{}_{}", photo.id, photo_type, size);
    }

    format!("{}_{}", photo.id, size)
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to upload file: {}", source))]
    Upload { source: SdkError<PutObjectError> },

    #[snafu(display("Failed to download file: {}", source))]
    Download { source: SdkError<GetObjectError> },

    #[snafu(display("Failed to remove file: {}", source))]
    Remove { source: SdkError<DeleteObjectError> },
}
