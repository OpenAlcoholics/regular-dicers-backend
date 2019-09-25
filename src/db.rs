#[database("primary_db")]
pub struct PrimaryDb(pub diesel::PgConnection);
