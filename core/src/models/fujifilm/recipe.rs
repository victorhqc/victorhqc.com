use fuji::recipe::{
    FujifilmRecipe as _FujifilmRecipe, FujifilmRecipeDetails, Settings as _Settings,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct FujifilmRecipe {
    pub id: String,
    pub name: String,
    pub author: String,
    pub src: String,
    pub inner: _FujifilmRecipe,
}

#[derive(Debug, Clone)]
pub struct Settings(pub _Settings);

impl FujifilmRecipe {
    pub fn new(name: String, recipe: _FujifilmRecipe) -> Self {
        let id = Uuid::new_v4().to_string();

        FujifilmRecipe {
            id,
            name,
            author: "todo".to_string(),
            src: "todo".to_string(),
            inner: recipe,
        }
    }

    pub fn from_db(
        id: String,
        name: String,
        author: String,
        src: String,
        recipe: _FujifilmRecipe,
    ) -> Self {
        FujifilmRecipe {
            id,
            name,
            author,
            src,
            inner: recipe,
        }
    }

    pub fn details(&self) -> &FujifilmRecipeDetails {
        &self.inner.details
    }
}
