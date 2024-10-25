use core_victorhqc_com::aws::image_size::ImageSize;
use std::sync::{Arc, Mutex};

pub type CachedImage = (PhotoId, ImageSize, Vec<u8>);
pub type PhotoId = String;

pub struct ImageCache {
    images: Arc<Mutex<Vec<CachedImage>>>,
}

impl Default for ImageCache {
    fn default() -> Self {
        ImageCache {
            images: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl ImageCache {
    pub fn get(&self, id: &PhotoId, size: &ImageSize) -> Option<Vec<u8>> {
        let images = self.images.lock().unwrap();
        let index = images.iter().position(|(i, s, _)| i == id && s == size);

        match index {
            None => None,
            Some(i) => {
                let image = images.get(i).map(|(_, _, v)| v.clone());

                image
            }
        }
    }

    pub fn save(&self, id: &PhotoId, size: &ImageSize, data: Vec<u8>) {
        let mut images = self.images.lock().unwrap();
        let index = images.iter().position(|(i, s, _)| i == id && s == size);

        match index {
            Some(i) => images[i].2 = data,
            None => images.push((id.clone(), size.clone(), data)),
        }
    }
}
