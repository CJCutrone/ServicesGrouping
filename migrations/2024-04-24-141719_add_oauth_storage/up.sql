CREATE TABLE "accounts" (
    "id" UUID PRIMARY KEY,
    "planning_center_id" VARCHAR(255) NOT NULL,
    "access_token" VARCHAR(255) NOT NULL,
    "refresh_token" VARCHAR(255) NOT NULL
);