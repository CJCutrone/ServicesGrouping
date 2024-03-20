use diesel::PgConnection;

pub mod load;
pub mod save;

pub fn process(file: &str, connection: &mut PgConnection) {
    let users = load::user::from(file);
    let groups = load::group::from(file);
    let group_assignments = load::group_assignment::from(file);

    save::user::to_database(connection, &users);
    save::group::to_database(connection, &groups);
    save::group_assignment::to_database(connection, &group_assignments);
}