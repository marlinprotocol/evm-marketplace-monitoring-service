use diesel::prelude::*;
use crate::schema::reachability_errors;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = reachability_errors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReachabilityError {
    pub id: i32,
    pub job: String,
    pub operator: String,
    pub ip: String,
    pub error: String,
    pub timestamp: i64,
}

#[derive(Insertable)]
#[diesel(table_name = reachability_errors)]
pub struct NewReachabilityError {
    pub job: String,
    pub operator: String,
    pub ip: String,
    pub error: String,
    pub timestamp: i64,
}

impl NewReachabilityError {
    pub fn new(job: String, operator: String, ip: String, error: String) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i64;

        Self {
            job,
            operator,
            ip,
            error,
            timestamp,
        }
    }

    pub fn insert(&self, conn: &mut PgConnection) -> QueryResult<ReachabilityError> {
        diesel::insert_into(reachability_errors::table)
            .values(self)
            .get_result(conn)
    }
}
