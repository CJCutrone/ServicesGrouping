// @generated automatically by Diesel CLI.

diesel::table! {
    group_assignments (id) {
        id -> Uuid,
        user_id -> Uuid,
        group_id -> Uuid,
        tickets -> Int4,
    }
}

diesel::table! {
    groups (id) {
        id -> Uuid,
        #[max_length = 255]
        planning_center_id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        positions -> Int4,
        is_deleted -> Bool,
    }
}

diesel::table! {
    service_dates (id) {
        id -> Uuid,
        group_assignment_id -> Uuid,
        tickets_consumed -> Int4,
        for_date -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        planning_center_id -> Varchar,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        is_deleted -> Bool,
    }
}

diesel::joinable!(group_assignments -> groups (group_id));
diesel::joinable!(group_assignments -> users (user_id));
diesel::joinable!(service_dates -> group_assignments (group_assignment_id));

diesel::allow_tables_to_appear_in_same_query!(
    group_assignments,
    groups,
    service_dates,
    users,
);
