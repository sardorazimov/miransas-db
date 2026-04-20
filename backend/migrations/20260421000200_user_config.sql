CREATE TABLE _miransas.project_user_config (
    project_id         UUID        PRIMARY KEY REFERENCES _miransas.projects(id) ON DELETE CASCADE,
    users_table        TEXT        NOT NULL,
    id_column          TEXT        NOT NULL DEFAULT 'id',
    email_column       TEXT,
    username_column    TEXT,
    password_column    TEXT,
    banned_column      TEXT,
    password_algorithm TEXT        NOT NULL DEFAULT 'bcrypt',
    searchable_columns TEXT[]      NOT NULL DEFAULT '{}',
    created_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_project_user_config_updated_at
    BEFORE UPDATE ON _miransas.project_user_config
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();
