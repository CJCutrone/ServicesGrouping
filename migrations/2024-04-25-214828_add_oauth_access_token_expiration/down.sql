CREATE TABLE "accounts" (
    "id" UUID PRIMARY KEY,
    "planning_center_id" VARCHAR(255) NOT NULL,
    "access_token" VARCHAR(255) NOT NULL,
    "refresh_token" VARCHAR(255) NOT NULL
);

INSERT INTO "accounts" (
    "id", 
    "planning_center_id", 
    "access_token", 
    "refresh_token"
)
SELECT 
    p."id", 
    p."planning_center_id", 
    p."access_token", 
    p."refresh_token"
FROM "planning_center_access_tokens" p;

DROP TABLE "planning_center_access_tokens";