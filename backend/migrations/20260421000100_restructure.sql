-- Restructure: move all internal tables to _miransas schema,
-- add schema_name to projects, drop databases table,
-- add query_history and saved_queries tables.

CREATE SCHEMA IF NOT EXISTS _miransas;

ALTER TABLE projects    SET SCHEMA _miransas;
ALTER TABLE databases   SET SCHEMA _miransas;
ALTER TABLE secrets     SET SCHEMA _miransas;
ALTER TABLE audit_logs  SET SCHEMA _miransas;

-- Add schema_name column to projects
ALTER TABLE _miransas.projects ADD COLUMN schema_name TEXT;

UPDATE _miransas.projects
   SET schema_name = 'proj_' || lower(regexp_replace(name, '[^a-zA-Z0-9]+', '_', 'g'));

ALTER TABLE _miransas.projects ALTER COLUMN schema_name SET NOT NULL;
ALTER TABLE _miransas.projects ADD CONSTRAINT projects_schema_name_unique UNIQUE (schema_name);

ALTER TABLE _miransas.projects DROP COLUMN IF EXISTS connection_string_encrypted;

-- Drop databases table (no longer needed)
DROP TABLE _miransas.databases;

-- Query history
CREATE TABLE _miransas.query_history (
    id            UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id    UUID        NOT NULL REFERENCES _miransas.projects(id) ON DELETE CASCADE,
    sql           TEXT        NOT NULL,
    duration_ms   INTEGER     NOT NULL,
    rows_affected BIGINT,
    success       BOOLEAN     NOT NULL,
    error_message TEXT,
    executed_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_query_history_project
    ON _miransas.query_history(project_id, executed_at DESC);

-- Saved queries
CREATE TABLE _miransas.saved_queries (
    id         UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID        NOT NULL REFERENCES _miransas.projects(id) ON DELETE CASCADE,
    name       TEXT        NOT NULL,
    sql        TEXT        NOT NULL,
    notes      TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_saved_queries_project
    ON _miransas.saved_queries(project_id);
CREATE TRIGGER set_saved_queries_updated_at
    BEFORE UPDATE ON _miransas.saved_queries
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();
