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
    let hd_pairs = futures::join!(
        s3.upload_to_aws_s3(
            (photo, &ImageSize::Hd, Some(&ImageType::Jpeg)),
            buffers.hd.jpeg
        ),
        s3.upload_to_aws_s3(
            (photo, &ImageSize::Hd, Some(&ImageType::Webp)),
            buffers.hd.webp
        )
    );

    let md_pairs = futures::join!(
        s3.upload_to_aws_s3(
            (photo, &ImageSize::Md, Some(&ImageType::Jpeg)),
            buffers.md.jpeg
        ),
        s3.upload_to_aws_s3(
            (photo, &ImageSize::Md, Some(&ImageType::Webp)),
            buffers.md.webp
        )
    );

    let sm_pairs = futures::join!(
        s3.upload_to_aws_s3(
            (photo, &ImageSize::Sm, Some(&ImageType::Jpeg)),
            buffers.sm.jpeg
        ),
        s3.upload_to_aws_s3(
            (photo, &ImageSize::Sm, Some(&ImageType::Webp)),
            buffers.sm.webp
        ),
    );

    hd_pairs.0.context(UploadSnafu {
        size: ImageSize::Hd,
        kind: ImageType::Jpeg,
    })?;

    hd_pairs.1.context(UploadSnafu {
        size: ImageSize::Hd,
        kind: ImageType::Webp,
    })?;

    md_pairs.0.context(UploadSnafu {
        size: ImageSize::Md,
        kind: ImageType::Jpeg,
    })?;

    md_pairs.1.context(UploadSnafu {
        size: ImageSize::Md,
        kind: ImageType::Webp,
    })?;

    sm_pairs.0.context(UploadSnafu {
        size: ImageSize::Sm,
        kind: ImageType::Jpeg,
    })?;

    sm_pairs.1.context(UploadSnafu {
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
