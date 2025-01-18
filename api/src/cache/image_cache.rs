use core_victorhqc_com::aws::{
    image_size::ImageSize,
    photo::{ByteStreamError, Error as AWSError},
    S3,
};
use core_victorhqc_com::models::photo::Photo;
use rocket::futures::lock::{Mutex, MutexGuard};
use snafu::prelude::*;
use std::iter::Iterator;
use std::sync::Arc;

pub struct CachedImage {
    id: PhotoId,
    size: ImageSize,
    bytes: Vec<u8>,
    md5_hash: String,
}

impl CachedImage {
    pub fn new(id: PhotoId, size: ImageSize, bytes: Vec<u8>) -> Self {
        let md5_hash = format!("{:x}", md5::compute(&bytes));

        Self {
            id,
            size,
            bytes,
            md5_hash,
        }
    }

    pub fn save_bytes(&mut self, bytes: Vec<u8>) {
        self.bytes = bytes;
    }

    pub fn get_md5(&self) -> String {
        self.md5_hash.clone()
    }
}

pub type PhotoId = String;

#[derive(Clone)]
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

    pub async fn md5_exists(&self, value: &str) -> bool {
        let images = self.images.lock().await;

        images.iter().any(|p| p.md5_hash == value)
    }

    pub async fn get(&self, photo: Photo, size: &ImageSize) -> Result<(String, Vec<u8>), Error> {
        let mut images = self.images.lock().await;
        let index = images
            .iter()
            .position(|p| p.id == photo.id && &p.size == size);

        match index {
            None => {
                debug!("Image not found in cache, downloading...");

                let response = self
                    .s3
                    .download_from_aws_s3((&photo, size))
                    .await
                    .context(GetAWSObjectSnafu)?;

                let data = response.body.collect().await.context(StreamSnafu)?;
                let bytes = data.into_bytes().to_vec();

                let hash = self.inner_save(&photo.id, size, bytes.clone(), &mut images);
                Ok((hash, bytes))
            }
            Some(i) => {
                // TODO: Could this be done without cloning the image?
                let (hash, bytes) = images
                    .get(i)
                    .map(|p| (p.get_md5(), p.bytes.clone()))
                    .unwrap();

                Ok((hash, bytes))
            }
        }
    }

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
    ) -> String {
        let index = images.iter().position(|p| &p.id == id && &p.size == size);

        match index {
            Some(i) => {
                images[i].save_bytes(data);
                images[i].get_md5()
            }
            None => {
                let cached = CachedImage::new(id.clone(), size.clone(), data);
                let hash = cached.get_md5();

                images.push(cached);

                hash
            }
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
