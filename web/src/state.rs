use crate::gql::get_portfolio;

#[derive(Debug)]
pub struct AppState {
    pub api_host: String,
    pub portfolio_photos: Vec<get_portfolio::GetPortfolioPhotos>,
}
