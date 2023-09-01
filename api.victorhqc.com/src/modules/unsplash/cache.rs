use super::entities::Picture;
use rand::Rng;
use std::time::SystemTime;

pub struct PicturesCache {
    pictures: Vec<CachedPicture>,
    max_len: usize,
    invalidate_at: u64,
}

pub struct ShouldFetch {
    pub index: usize,
    pub result: FetchResult,
}

#[derive(PartialEq)]
pub enum FetchResult {
    Replace,
    New,
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

    pub fn should_fetch(&self, query: &str, orientation: &str) -> (bool, ShouldFetch) {
        // Should always fetch a new picture when the amount of pictures is less than the max it
        // cah hold.
        if self.get_cache_size(query, orientation) < self.max_len {
            return (
                true,
                ShouldFetch {
                    result: FetchResult::New,
                    index: 0,
                },
            );
        }

        let mut should_fetch = true;
        for (pos, picture) in self.pictures.iter().enumerate() {
            if query == picture.query && orientation == picture.orientation {
                let elapsed = picture.last_updated.elapsed().unwrap();

                if elapsed.as_secs() >= 60 * self.invalidate_at {
                    debug!("Fetching a new picture");
                    return (
                        true,
                        ShouldFetch {
                            result: FetchResult::Replace,
                            index: pos,
                        },
                    );
                }

                debug!(
                    "Last update happened {} secs ago, skipping fetch for another {} secs",
                    elapsed.as_secs(),
                    60 * self.invalidate_at - elapsed.as_secs()
                );

                should_fetch = false;
            }
        }

        return (
            should_fetch,
            ShouldFetch {
                result: FetchResult::New,
                index: self.pictures.len(),
            },
        );
    }

    pub fn get_last_random<'a>(&'a self, query: &str, orientation: &str) -> Option<&'a Picture> {
        let mut rng = rand::thread_rng();

        let mut possible = vec![];
        for (index, picture) in self.pictures.iter().enumerate() {
            if query == picture.query && orientation == picture.orientation {
                possible.push(index);
            }
        }

        if possible.len() == 0 {
            return None;
        }

        let random_index: usize = rng.gen_range(0..possible.len());
        let random_index = possible[random_index];

        match self.pictures.get(random_index) {
            Some(picture) => Some(&picture.picture),
            None => None,
        }
    }

    pub fn set_random_picture(&mut self, picture: Picture, query: &str, orientation: &str) {
        let last_updated = SystemTime::now();

        let new_picture = CachedPicture {
            picture,
            last_updated,
            query: query.into(),
            orientation: orientation.into(),
        };

        let (should_replace, r) = self.should_fetch(query, orientation);

        if should_replace == true && r.result == FetchResult::Replace {
            debug!("Replacing cached picture on index {}", r.index);
            self.pictures[r.index] = new_picture;
        } else if self.get_cache_size(query, orientation) <= self.max_len {
            self.pictures.push(new_picture);
            debug!(
                "Pushed a new picture to the cache, current amount: {}",
                self.pictures.len()
            );
        } else {
            debug!("Uh oh, cache is full, not allowed to push a new picture");
        }
    }

    fn get_cache_size(&self, query: &str, orientation: &str) -> usize {
        let mut size: usize = 0;

        for picture in &self.pictures {
            if picture.query != query || picture.orientation != orientation {
                continue;
            }

            size += 1;
        }

        return size;
    }
}
