ALTER TABLE
    roles
DROP COLUMN
    permissions,
ADD COLUMN
    permissions BIGINT NOT NULL DEFAULT 0;
