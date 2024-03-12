CREATE TABLE "users" (
    "id" UUID PRIMARY KEY,
    "planning_center_id" INT NOT NULL,
    "first_name" VARCHAR(255) NOT NULL,
    "last_name" VARCHAR(255) NOT NULL
);

CREATE TABLE "groups" (
    "id" UUID PRIMARY KEY,
    "planning_center_id" INT NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "positions" INT NOT NULL
);

CREATE TABLE "group_assignments" (
    "id" UUID PRIMARY KEY,
    "user_id" UUID NOT NULL REFERENCES "users" ("id"),
    "group_id" UUID NOT NULL REFERENCES "groups" ("id")
)