DROP INDEX IF EXISTS idx_sessions_last_activity;

DROP INDEX IF EXISTS idx_sessions_created_at;

DROP INDEX IF EXISTS idx_visits_path_id;

DROP INDEX IF EXISTS idx_visits_created_at;

DROP INDEX IF EXISTS idx_visits_session_id;

DROP INDEX IF EXISTS idx_paths_path;

DROP TABLE IF EXISTS visits;

DROP TABLE IF EXISTS paths;

DROP TABLE IF EXISTS sessions;
