use async_graphql::{SimpleObject, ID};
use crate::models::fujifilm::{FilmSimulation, FujifilmRecipe as FujifilmRecipeModel};

#[derive(SimpleObject, Clone)]
pub struct FujifilmRecipe {
    pub id: ID,
    pub name: String,
    pub src: String,
    pub film_simulation: String,
}

impl From<FujifilmRecipeModel> for FujifilmRecipe {
    fn from(model: FujifilmRecipeModel) -> Self {
        FujifilmRecipe {
            id: model.id.into(),
            name: model.name,
            src: model.src,
            film_simulation: model.film_simulation.to_string(),
        }
    }
}
