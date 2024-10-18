use crate::graphql::{context::get_conn, models::FujifilmRecipe as GQLFujifilmRecipe};
use async_graphql::{Context, Object, Result};
use core_victorhqc_com::models::fujifilm::FujifilmRecipe;

#[derive(Default)]
pub struct FujifilmRecipeQuery;

#[Object]
impl FujifilmRecipeQuery {
    pub async fn fujifilm_recipes(
        &self,
        ctx: &Context<'_>,
        film_simulation: String,
    ) -> Result<Vec<GQLFujifilmRecipe>> {
        let mut conn = get_conn(ctx).await?;
        let recipes = FujifilmRecipe::find_by_film_simulation(&mut conn, &film_simulation).await?;
        let recipes = recipes.into_iter().map(|r| r.into()).collect();

        Ok(recipes)
    }
}
