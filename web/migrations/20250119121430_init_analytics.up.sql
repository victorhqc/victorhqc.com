CREATE TABLE IF NOT EXISTS sessions (
  id TEXT PRIMARY KEY NOT NULL CHECK (length (id) = 36),
  user_agent TEXT,
  created_at TIMESTAMP DEFAULT current_timestamp NOT NULL CHECK (created_at <= current_timestamp),
  last_activity_at TIMESTAMP DEFAULT current_timestamp NOT NULL
);

CREATE TABLE IF NOT EXISTS paths (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  path TEXT NOT NULL UNIQUE CHECK (length (path) > 0)
);

CREATE TABLE IF NOT EXISTS visits (
  id TEXT PRIMARY KEY NOT NULL CHECK (length (id) = 36),
  session_id TEXT NOT NULL,
  path_id INTEGER NOT NULL,
  referer TEXT,
  created_at TIMESTAMP DEFAULT current_timestamp NOT NULL CHECK (created_at <= current_timestamp),
  FOREIGN KEY (session_id) REFERENCES sessions (id) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (path_id) REFERENCES paths (id) ON DELETE RESTRICT ON UPDATE CASCADE
);

CREATE UNIQUE INDEX idx_paths_path ON paths (path);

CREATE INDEX idx_visits_session_id ON visits (session_id);

CREATE INDEX idx_visits_created_at ON visits (created_at);

CREATE INDEX idx_visits_path_id ON visits (path_id);

CREATE INDEX idx_sessions_created_at ON sessions (created_at);

CREATE INDEX idx_sessions_last_activity ON sessions (last_activity_at);
