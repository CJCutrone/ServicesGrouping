CREATE TABLE "planning_center_access_tokens" (
    "id" UUID PRIMARY KEY,
    "planning_center_id" VARCHAR(255) NOT NULL,
    "access_token" VARCHAR(255) NOT NULL,
    "refresh_token" VARCHAR(255) NOT NULL,
    "expires_at" BIGINT NOT NULL
);

INSERT INTO "planning_center_access_tokens" (
    "id", 
    "planning_center_id", 
    "access_token", 
    "refresh_token", 
    "expires_at"
)
SELECT 
    a."id", 
    a."planning_center_id", 
    a."access_token", 
    a."refresh_token", 
    0
FROM "accounts" a;

DROP TABLE "accounts";