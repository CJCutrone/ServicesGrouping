CREATE VIEW "assigned_dates"
AS
SELECT
    sd."id" as service_date_id,
    sd."for_date" as service_date,
    sd."tickets_consumed" as tickets_consumed,
    g."name" as group_name,
    u."first_name" as user_first_name,
    u."last_name" as user_last_name
FROM "service_dates" sd
INNER JOIN "group_assignments" ga
    ON sd."group_assignment_id" = ga."id"
INNER JOIN "groups" g
    ON ga."group_id" = g."id"
INNER JOIN "users" u
    ON ga."user_id" = u."id"
;

CREATE VIEW "ticket_pool"
AS
SELECT
    ga."id" as group_assignment_id,
    ga."tickets" as available_tickets,
    g."name" as group_name,
    u."first_name" as user_first_name,
    u."last_name" as user_last_name
FROM "group_assignments" ga
INNER JOIN "groups" g
    ON ga."group_id" = g."id"
INNER JOIN "users" u
    ON ga."user_id" = u."id"
;