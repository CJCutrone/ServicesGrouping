ALTER TABLE "groups" 
    ALTER COLUMN "planning_center_id" TYPE INT USING("planning_center_id"::integer);
ALTER TABLE "users" 
    ALTER COLUMN "planning_center_id" TYPE INT USING("planning_center_id"::integer);