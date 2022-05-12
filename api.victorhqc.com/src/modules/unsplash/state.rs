use super::entities::Picture;
use std::time::SystemTime;

pub struct PicturesState {
    random: Option<RandomPicture>,
}

pub struct RandomPicture {
    picture: Picture,
    last_updated: SystemTime,
}

impl PicturesState {
    pub fn new() -> Self {
        Self { random: None }
    }

    pub fn should_fetch(&self) -> bool {
        match &self.random {
            None => {
                debug!("Fetching first picture");
                return true;
            }
            Some(p) => {
                let elapsed = p.last_updated.elapsed().unwrap();

                if elapsed.as_secs() >= 60 {
                    debug!("Fetching a new picture");
                    return true;
                }

                debug!(
                    "Last update happened {} secs ago, skipping fetch",
                    elapsed.as_secs()
                );

                return false;
            }
        };
    }

    pub fn get_last_random(&self) -> Option<&Picture> {
        match &self.random {
            Some(p) => Some(&p.picture),
            None => None,
        }
    }

    pub fn set_random_picture(&mut self, picture: Picture) {
        let last_updated = SystemTime::now();

        self.random = Some(RandomPicture {
            picture,
            last_updated,
        });
    }
}
