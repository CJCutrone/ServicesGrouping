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
        planning_center_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        positions -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        planning_center_id -> Int4,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
    }
}

diesel::joinable!(group_assignments -> groups (group_id));
diesel::joinable!(group_assignments -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    group_assignments,
    groups,
    users,
);
