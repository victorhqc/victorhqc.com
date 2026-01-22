-- Useful SQL commands for analytics DB

-- 1. Total Visits
SELECT COUNT(*) AS total_visits
FROM visits;

-- 2. Total Visits by User Agent
SELECT
  COALESCE(s.user_agent, 'Unknown') AS user_agent,
  COUNT(v.id) AS total_visits
FROM visits v
JOIN sessions s ON v.session_id = s.id
GROUP BY s.user_agent
ORDER BY total_visits DESC;

-- 3. Total Visits by Referer
SELECT
  COALESCE(referer, 'Direct / None') AS referer,
  COUNT(*) AS total_visits
FROM visits
GROUP BY referer
ORDER BY total_visits DESC;

-- 4. Total Visits grouped by year-month
SELECT
  strftime('%Y-%m', created_at) AS year_month,
  COUNT(*) AS total_visits
FROM visits
GROUP BY year_month
ORDER BY year_month DESC;

-- 5. Total Visits per path
SELECT
  p.path,
  COUNT(v.id) AS total_visits
FROM visits v
JOIN paths p ON v.path_id = p.id
GROUP BY p.path
ORDER BY total_visits DESC;

-- 6. Total Visits per path and grouped by year-month
SELECT
  p.path,
  strftime('%Y-%m', v.created_at) AS year_month,
  COUNT(v.id) AS total_visits
FROM visits v
JOIN paths p ON v.path_id = p.id
GROUP BY p.path, year_month
ORDER BY year_month DESC, total_visits DESC;

-- Total unique sessions
SELECT COUNT(DISTINCT session_id) AS unique_sessions
FROM visits;

-- Average visits per session
SELECT
  ROUND(CAST(COUNT(v.id) AS FLOAT) / COUNT(DISTINCT v.session_id), 2) AS avg_visits_per_session
FROM visits v;

-- Top 10 most popular paths
SELECT
  p.path,
  COUNT(v.id) AS total_visits
FROM visits v
JOIN paths p ON v.path_id = p.id
GROUP BY p.path
ORDER BY total_visits DESC
LIMIT 10;

-- Top 10 most popular photos
SELECT
  p.path,
  COUNT(v.id) AS total_visits
FROM visits v
JOIN paths p ON v.path_id = p.id
WHERE p.path LIKE '/photo/%'
GROUP BY p.path
ORDER BY total_visits DESC
LIMIT 10;

-- Visit trends (last 12 months)
SELECT
  strftime('%Y-%m', created_at) AS year_month,
  COUNT(*) AS total_visits
FROM visits
WHERE created_at >= datetime('now', '-12 months')
GROUP BY year_month
ORDER BY year_month ASC;

-- Sessions with their visit count
SELECT
  s.id,
  s.user_agent,
  COUNT(v.id) AS visit_count,
  s.created_at,
  s.last_activity_at
FROM sessions s
LEFT JOIN visits v ON s.id = v.session_id
GROUP BY s.id
ORDER BY visit_count DESC;
