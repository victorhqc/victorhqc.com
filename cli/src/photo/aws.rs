use crate::photo::build_images::ImageBuffers;
use core_victorhqc_com::{
    aws::{
        image_size::{ImageSize, ImageType},
        photo::Error as AWSError,
        S3,
    },
    models::photo::Photo,
};
use log::error;
use snafu::prelude::*;

pub async fn upload(photo: &Photo, s3: &S3, buffers: ImageBuffers) -> Result<(), Error> {
    let hd_pairs = futures::join!(
        s3.upload_to_aws_s3((photo, &ImageSize::Hd, &ImageType::Jpeg), buffers.hd.jpeg),
        s3.upload_to_aws_s3((photo, &ImageSize::Hd, &ImageType::Webp), buffers.hd.webp)
    );

    let md_pairs = futures::join!(
        s3.upload_to_aws_s3((photo, &ImageSize::Md, &ImageType::Jpeg), buffers.md.jpeg),
        s3.upload_to_aws_s3((photo, &ImageSize::Md, &ImageType::Webp), buffers.md.webp)
    );

    let sm_pairs = futures::join!(
        s3.upload_to_aws_s3((photo, &ImageSize::Sm, &ImageType::Jpeg), buffers.sm.jpeg),
        s3.upload_to_aws_s3((photo, &ImageSize::Sm, &ImageType::Webp), buffers.sm.webp),
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

pub async fn remove(photo: &Photo, s3: &S3) -> Result<(), Error> {
    let hd_pairs = futures::join!(
        s3.remove_from_aws_s3((photo, &ImageSize::Hd, &ImageType::Jpeg)),
        s3.remove_from_aws_s3((photo, &ImageSize::Hd, &ImageType::Webp))
    );

    let md_pairs = futures::join!(
        s3.remove_from_aws_s3((photo, &ImageSize::Md, &ImageType::Jpeg)),
        s3.remove_from_aws_s3((photo, &ImageSize::Md, &ImageType::Webp))
    );

    let sm_pairs = futures::join!(
        s3.remove_from_aws_s3((photo, &ImageSize::Sm, &ImageType::Jpeg)),
        s3.remove_from_aws_s3((photo, &ImageSize::Sm, &ImageType::Webp)),
    );

    hd_pairs
        .0
        .context(RemoveSnafu {
            size: ImageSize::Hd,
            kind: ImageType::Jpeg,
        })
        .map_err(|err| error!("{}", err))
        .ok();

    hd_pairs
        .1
        .context(RemoveSnafu {
            size: ImageSize::Hd,
            kind: ImageType::Webp,
        })
        .map_err(|err| error!("{}", err))
        .ok();

    md_pairs
        .0
        .context(RemoveSnafu {
            size: ImageSize::Md,
            kind: ImageType::Jpeg,
        })
        .map_err(|err| error!("{}", err))
        .ok();

    md_pairs
        .1
        .context(RemoveSnafu {
            size: ImageSize::Md,
            kind: ImageType::Webp,
        })
        .map_err(|err| error!("{}", err))
        .ok();

    sm_pairs
        .0
        .context(RemoveSnafu {
            size: ImageSize::Sm,
            kind: ImageType::Jpeg,
        })
        .map_err(|err| error!("{}", err))
        .ok();

    sm_pairs
        .1
        .context(RemoveSnafu {
            size: ImageSize::Sm,
            kind: ImageType::Webp,
        })
        .map_err(|err| error!("{}", err))
        .ok();

    Ok(())
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to upload {} photo of type {}: {}", kind, size, source))]
    Upload {
        size: ImageSize,
        kind: ImageType,
        source: AWSError,
    },

    #[snafu(display("Failed to remove {} photo of type {}: {}", kind, size, source))]
    Remove {
        size: ImageSize,
        kind: ImageType,
        source: AWSError,
    },
}
