CREATE TABLE IF NOT EXISTS fuji_recipes (
  id TEXT PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  author TEXT NOT NULL,
  sensor TEXT NOT NULL,
  src TEXT NOT NULL,
  film_simulation TEXT NOT NULL,
  white_balance TEXT NOT NULL,
  white_balance_shift TEXT NOT NULL,
  dynamic_range TEXT NOT NULL,
  d_range_priority TEXT NULL,
  highlight_tone REAL NOT NULL,
  shadow_tone REAL NOT NULL,
  color INTEGER NOT NULL,
  sharpness INTEGER NOT NULL,
  clarity INTEGER NULL,
  high_iso_noise_reduction INTEGER NOT NULL,
  grain_strength TEXT NULL,
  grain_size TEXT NULL,
  color_chrome_effect TEXT NULL,
  color_chrome_fx_blue TEXT NULL,
  monochromatic_color TEXT NULL
);

CREATE TABLE IF NOT EXISTS photos (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    src TEXT NOT NULL UNIQUE,
    filename TEXT NOT NULL UNIQUE,
    filetype TEXT NOT NULL,
    rating INTEGER NOT NULL,
    date_taken DATE NULL,
    city TEXT NULL,
    created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
    updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT FALSE
);

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
  FOREIGN KEY (fuji_recipe_id)
    REFERENCES fuji_recipes (id)
    ON DELETE SET NULL
    ON UPDATE CASCADE,
  FOREIGN KEY (photo_id)
    REFERENCES photos (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  UNIQUE (photo_id)
);

CREATE TABLE IF NOT EXISTS tags (
  id TEXT PRIMARY KEY NOT NULL,
  name TEXT NOT NULL UNIQUE,
  created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
  updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
  deleted BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS photo_tags (
  id TEXT PRIMARY KEY NOT NULL,
  tag_id TEXT NOT NULL,
  photo_id TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
  updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
  FOREIGN KEY (tag_id)
    REFERENCES tags (id)
      ON DELETE CASCADE
      ON UPDATE CASCADE,
  FOREIGN KEY (photo_id)
    REFERENCES photos (id)
      ON DELETE CASCADE
      ON UPDATE CASCADE
);
