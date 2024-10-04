-- Add up migration script here

CREATE TABLE IF NOT EXISTS films (
  id TEXT PRIMARY KEY NOT NULL,
  name TEXT NOT NULL UNIQUE,
  iso INTEGER NOT NULL,
  maker TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
  updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL
);

ALTER TABLE exif_metas RENAME TO exif_metas_OLD;

CREATE TABLE IF NOT EXISTS exif_metas (
  id TEXT PRIMARY KEY NOT NULL,
  iso INTEGER NOT NULL,
  focal_length REAL NOT NULL,
  exposure_compensation REAL NOT NULL,
  aperture REAL NOT NULL,
  maker TEXT NOT NULL,
  crop_factor REAL NOT NULL,
  camera_name TEXT NOT NULL,
  lens_name TEXT NULL,
  photo_id TEXT NOT NULL,
  fuji_recipe_id TEXT NULL,
  film_id TEXT NULL,
  FOREIGN KEY (fuji_recipe_id)
    REFERENCES fuji_recipes (id)
    ON DELETE SET NULL
    ON UPDATE CASCADE,
  FOREIGN KEY (photo_id)
    REFERENCES photos (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  UNIQUE (photo_id),
  FOREIGN KEY (film_id)
    REFERENCES films (id)
    ON DELETE SET NULL
    ON UPDATE CASCADE
);

INSERT INTO
exif_metas (
  id,
  iso,
  focal_length,
  exposure_compensation,
  aperture,
  maker,
  crop_factor,
  camera_name,
  lens_name,
  photo_id,
  fuji_recipe_id
)
SELECT
  id,
  iso,
  focal_length,
  exposure_compensation,
  aperture,
  maker,
  crop_factor,
  camera_name,
  lens_name,
  photo_id,
  fuji_recipe_id
FROM exif_metas_OLD;

DROP TABLE IF EXISTS exif_metas_OLD;

PRAGMA foreign_keys = ON;
