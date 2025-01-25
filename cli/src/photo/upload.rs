use crate::photo::build_images::ImageBuffers;
use core_victorhqc_com::{
    aws::{
        image_size::{ImageSize, ImageType},
        photo::Error as UploadError,
        S3,
    },
    models::photo::Photo,
};
use snafu::prelude::*;

pub async fn upload(photo: &Photo, s3: &S3, buffers: ImageBuffers) -> Result<(), Error> {
    let (result_hd, result_hd_webp, result_md, result_md_webp, result_sm, result_sm_webp) = futures::join!(
        s3.upload_to_aws_s3(
            (photo, &ImageSize::Hd, Some(&ImageType::Jpeg)),
            buffers.hd.jpeg
        ),
        s3.upload_to_aws_s3(
            (photo, &ImageSize::Hd, Some(&ImageType::Webp)),
            buffers.hd.webp
        ),
        s3.upload_to_aws_s3(
            (photo, &ImageSize::Md, Some(&ImageType::Jpeg)),
            buffers.md.jpeg
        ),
        s3.upload_to_aws_s3(
            (photo, &ImageSize::Md, Some(&ImageType::Webp)),
            buffers.md.webp
        ),
        s3.upload_to_aws_s3(
            (photo, &ImageSize::Sm, Some(&ImageType::Jpeg)),
            buffers.sm.jpeg
        ),
        s3.upload_to_aws_s3(
            (photo, &ImageSize::Sm, Some(&ImageType::Webp)),
            buffers.sm.webp
        ),
    );

    result_hd.context(UploadSnafu {
        size: ImageSize::Hd,
        kind: ImageType::Jpeg,
    })?;

    result_hd_webp.context(UploadSnafu {
        size: ImageSize::Hd,
        kind: ImageType::Webp,
    })?;

    result_md.context(UploadSnafu {
        size: ImageSize::Md,
        kind: ImageType::Jpeg,
    })?;

    result_md_webp.context(UploadSnafu {
        size: ImageSize::Md,
        kind: ImageType::Webp,
    })?;

    result_sm.context(UploadSnafu {
        size: ImageSize::Sm,
        kind: ImageType::Jpeg,
    })?;

    result_sm_webp.context(UploadSnafu {
        size: ImageSize::Sm,
        kind: ImageType::Webp,
    })?;

    Ok(())
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to upload {} photo of type {}: {}", kind, size, source))]
    Upload {
        size: ImageSize,
        kind: ImageType,
        source: UploadError,
    },
}
