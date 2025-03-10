use super::{session::Session, visit::Visit};
use log::debug;
use sqlx::SqlitePool;
use tokio::sync::mpsc::Receiver;

pub async fn store(pool: SqlitePool, mut rx: Receiver<(Session, Visit)>) {
    let mut conn = pool.acquire().await.unwrap();

    while let Some((session, visit)) = rx.recv().await {
        session.create_or_update(&mut conn).await.unwrap();
        visit.save(&mut conn).await.unwrap();

        debug!("Session: {:?}", session);
        debug!("Visit: {:?}", visit)
    }
}
