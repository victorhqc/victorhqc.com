use core_victorhqc_com::aws::{
    image_size::ImageSize,
    photo::{ByteStreamError, Error as AWSError},
    ByteStream, S3,
};
use core_victorhqc_com::models::photo::Photo;
use futures::lock::{Mutex, MutexGuard};
use snafu::prelude::*;
use std::iter::Iterator;
use std::sync::Arc;

pub type CachedImage = (PhotoId, ImageSize, Vec<u8>);
pub type PhotoId = String;

pub struct ImageCache {
    pub s3: S3,
    images: Arc<Mutex<Vec<CachedImage>>>,
}
impl ImageCache {
    pub fn default(s3: S3) -> Self {
        ImageCache {
            s3,
            images: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn stream(&self, photo: Photo, size: &ImageSize) -> Result<ByteStream, Error> {
        let mut images = self.images.lock().await;
        let index = images
            .iter()
            .position(|(i, s, _)| i == &photo.id && s == size);

        match index {
            None => {
                debug!("Image not found in cache, downloading...");

                let response = self
                    .s3
                    .download_from_aws_s3((&photo, &size))
                    .await
                    .context(GetAWSObjectSnafu)?;

                let data = response.body.collect().await.context(StreamSnafu)?;
                let bytes = data.into_bytes().to_vec();

                // TODO: Save Image while streaming
                self.inner_save(&photo.id, size, bytes.clone(), &mut images);
                Ok(ByteStream::from(bytes))
            }
            Some(i) => {
                // TODO: Could this be done without cloning the image?
                let bytes = images.get(i).map(|(_, _, v)| v.clone()).unwrap();

                Ok(ByteStream::from(bytes))
            }
        }
    }

    // pub async fn get(&self, id: &PhotoId, size: &ImageSize) -> Option<Vec<u8>> {
    //     let images = self.images.lock().await;
    //     let index = images.iter().position(|(i, s, _)| i == id && s == size);

    //     match index {
    //         None => None,
    //         Some(i) => images.get(i).map(|(_, _, v)| v.clone()),
    //     }
    // }

    pub async fn save(&self, id: &PhotoId, size: &ImageSize, data: Vec<u8>) {
        let mut images = self.images.lock().await;

        self.inner_save(id, size, data, &mut images);
    }

    fn inner_save(
        &self,
        id: &PhotoId,
        size: &ImageSize,
        data: Vec<u8>,
        images: &mut MutexGuard<'_, Vec<CachedImage>>,
    ) {
        let index = images.iter().position(|(i, s, _)| i == id && s == size);

        match index {
            Some(i) => images[i].2 = data,
            None => images.push((id.clone(), size.clone(), data)),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to get AWS Object: {}", source))]
    GetAWSObject { source: AWSError },

    #[snafu(display("Failed to download photo: {}", source))]
    Stream { source: ByteStreamError },
}
