use async_graphql::{Context as AsyncGraphqlContext, Error as AsyncGraphqlError};
use sqlx::{Pool, Sqlite};

#[derive(Clone)]
pub struct Context {
    pub pool: Pool<Sqlite>,
}

impl Context {
    pub fn default(pool: Pool<Sqlite>) -> Self {
        Context { pool }
    }
}

pub async fn get_pool<'a>(
    ctx: &'a AsyncGraphqlContext<'_>,
) -> Result<&'a Pool<Sqlite>, AsyncGraphqlError> {
    let Context { pool, .. } = ctx.data()?;
    Ok(pool)
}
