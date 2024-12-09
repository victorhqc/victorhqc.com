use crate::requests::photos::Photo;

#[derive(Debug)]
pub struct AppState {
    pub api_host: String,
    pub portfolio_photos: Vec<Photo>,
}
