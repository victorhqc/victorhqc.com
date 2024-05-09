CREATE TABLE fuji_recipes (
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

CREATE TABLE exif_metas (
  id TEXT PRIMARY KEY NOT NULL,
  iso INTEGER NOT NULL,
  focal_length INTEGER NOT NULL,
  aperture REAL NOT NULL,
  maker TEXT NOT NULL,
  crop_factor REAL NOT NULL,
  camera_name TEXT NOT NULL,
  lens_name TEXT NULL,
  fuji_recipe_id TEXT NULL,
  FOREIGN KEY (fuji_recipe_id)
    REFERENCES fuji_recipes (id)
      ON DELETE SET NULL
      ON UPDATE CASCADE
);

CREATE TABLE photos (
    id TEXT PRIMARY KEY NOT NULL,
    src TEXT NOT NULL UNIQUE,
    filename TEXT NOT NULL UNIQUE,
    filetype TEXT NOT NULL,
    date_taken DATE NULL,
    city TEXT NULL,
    created_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
    updated_at TIMESTAMP DEFAULT current_timestamp NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    exif_meta_id TEXT NOT NULL UNIQUE,
    FOREIGN KEY (exif_meta_id)
      REFERENCES exif_metas (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
