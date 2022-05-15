use super::entities::Picture;
use std::time::SystemTime;

pub struct PicturesCache {
    pictures: Vec<CachedPicture>,
    max_len: usize,
    invalidate_at: u64,
}

pub struct CachedPicture {
    picture: Picture,
    last_updated: SystemTime,
    query: String,
    orientation: String,
}

impl PicturesCache {
    pub fn new(max_len: usize, invalidate_at: u64) -> Self {
        Self {
            pictures: Vec::new(),
            max_len,
            invalidate_at,
        }
    }

    pub fn should_fetch(&self, query: &str, orientation: &str) -> bool {
        for pictures in &self.pictures {
            if query == pictures.query && orientation == pictures.orientation {
                let elapsed = pictures.last_updated.elapsed().unwrap();

                if elapsed.as_secs() >= 60 * self.invalidate_at {
                    debug!("Fetching a new picture");
                    return true;
                }

                debug!(
                    "Last update happened {} secs ago, skipping fetch for another {} secs",
                    elapsed.as_secs(),
                    60 * self.invalidate_at - elapsed.as_secs()
                );

                return false;
            }
        }

        debug!("Fetching first picture");
        return true;
    }

    pub fn get_last_random(&self, query: &str, orientation: &str) -> Option<&Picture> {
        for picture in &self.pictures {
            if query == picture.query && orientation == picture.orientation {
                return Some(&picture.picture);
            }
        }

        // If not found, get the first.
        if self.pictures.len() > 0 {
            return Some(&self.pictures[0].picture);
        }

        return None;
    }

    pub fn set_random_picture(&mut self, picture: Picture, query: &str, orientation: &str) {
        let last_updated = SystemTime::now();
        let mut index = 0;
        let mut should_replace = false;

        let new_picture = CachedPicture {
            picture,
            last_updated,
            query: query.into(),
            orientation: orientation.into(),
        };

        for (pos, picture) in self.pictures.iter().enumerate() {
            if query == picture.query && orientation == picture.orientation {
                should_replace = true;
                index = pos;
            }
        }

        if should_replace == true {
            debug!("Replacing cached picture on index {}", index);
            self.pictures[index] = new_picture;
        } else if self.pictures.len() <= self.max_len {
            self.pictures.push(new_picture);
            debug!(
                "Pushed a new picture to the cache, current amount: {}",
                self.pictures.len()
            );
        } else {
            debug!("Uh oh, cache is full, not allowed to push a new picture");
        }
    }
}
