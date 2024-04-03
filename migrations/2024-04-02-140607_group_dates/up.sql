CREATE TABLE "service_dates" (
    "id" UUID PRIMARY KEY,
    "group_assignment_id" UUID NOT NULL REFERENCES "group_assignments" ("id"),
    "tickets_consumed" INT NOT NULL,
    "for_date" TIMESTAMP NOT NULL
);