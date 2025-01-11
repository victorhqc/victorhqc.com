use crate::AppState;
use core_victorhqc_com::aws::image_size::ImageSize;
use core_victorhqc_com::models::{photo::Photo, tag::Tag};
use log::{debug, info};
use rocket::futures::future::join_all;
use rocket::tokio;
use std::collections::HashSet;

pub fn prepare_images(state: AppState, tags: Vec<String>) -> tokio::task::JoinHandle<AppState> {
    let sizes = [ImageSize::Hd, ImageSize::Md, ImageSize::Sm];

    info!("Preparing images...");
    tokio::spawn(async move {
        let mut conn = state.db_pool.acquire().await.unwrap();
        let tags: Vec<&str> = tags.iter().map(|t| t.as_str()).collect();

        let tags = Tag::find_by_names(&mut conn, &tags).await.unwrap();
        let tag_ids = tags.into_iter().map(|t| t.id).collect::<Vec<_>>();

        if tag_ids.is_empty() {
            return state;
        }

        let photos: Vec<(String, Photo)> = Photo::find_by_tag_ids(&mut conn, &tag_ids, None)
            .await
            .unwrap();
        let photos: Vec<Photo> = photos.into_iter().map(|(_, p)| p).collect();

        info!("Bootstrapping {} images", photos.len());

        // Removes repeated photos.
        let photos_set: HashSet<Photo> = HashSet::from_iter(photos);

        let download_futures = photos_set
            .iter()
            .flat_map(|photo| {
                sizes.iter().map(|img_size| {
                    let state = state.clone();
                    let photo = photo.clone();

                    async move {
                        let response = state
                            .img_cache
                            .s3
                            .download_from_aws_s3((&photo, img_size))
                            .await
                            .unwrap();

                        let data = response.body.collect().await.unwrap();
                        let bytes = data.into_bytes().to_vec();

                        state
                            .img_cache
                            .save(&photo.id, img_size, bytes.clone())
                            .await;

                        debug!("Cached photo {} in {}", &photo.id, img_size);
                    }
                })
            })
            .collect::<Vec<_>>();

        join_all(download_futures).await;

        state
    })
}
