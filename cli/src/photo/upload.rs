use crate::photo::build_images::ImageBuffers;
use core_victorhqc_com::{
    aws::{image_size::ImageSize, photo::Error as UploadError, S3},
    models::photo::Photo,
};
use snafu::prelude::*;

pub async fn upload(photo: &Photo, s3: &S3, buffers: ImageBuffers) -> Result<(), Error> {
    let (result_hd, result_md, result_sm) = futures::join!(
        s3.upload_to_aws_s3((photo, &ImageSize::Hd), buffers.hd),
        s3.upload_to_aws_s3((photo, &ImageSize::Md), buffers.md),
        s3.upload_to_aws_s3((photo, &ImageSize::Sm), buffers.sm),
    );

    result_hd.context(UploadSnafu {
        size: ImageSize::Hd,
    })?;

    result_md.context(UploadSnafu {
        size: ImageSize::Md,
    })?;

    result_sm.context(UploadSnafu {
        size: ImageSize::Sm,
    })?;

    Ok(())
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to upload {} Photo: {:?}", size, source))]
    Upload {
        size: ImageSize,
        source: UploadError,
    },
}
