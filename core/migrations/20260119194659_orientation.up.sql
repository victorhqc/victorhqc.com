ALTER TABLE photos
ADD COLUMN orientation TEXT NOT NULL DEFAULT 'landscape'
CHECK (orientation IN ('landscape', 'portrait'));
