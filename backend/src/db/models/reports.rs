use chrono::{NaiveDateTime, Utc};
use crate::utils;
use diesel::prelude::*;

#[derive(Debug, Identifiable, Serialize, Queryable, QueryableByName, Insertable, AsChangeset)]
#[table_name = "reports"]
#[changeset_options(treat_none_as_null="true")]
#[primary_key(uuid)]
pub struct Reports {
    pub uuid: String,
    pub user_uuid: String,

    pub created_at: String,

    pub report: String,
}

/// Local methods
impl Reports {
    pub fn new(user_uuid: String, report: String) -> Self {
        let now = Utc::now().naive_utc();
        
        Self {
            uuid: utils::create_uuid(),
            user_uuid: user_uuid,
            created_at: now.format("%Y-%m-%d %H:%M:%S").to_string(),
            report: report,
        }
    }
}

use crate::db::schema::reports;
use crate::db::DbConn;

/// Database Methods

impl Reports {
    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "uuid": self.uuid,
            "user_uuid": self.user_uuid,
            "created_at": self.created_at,
            "report": self.report,
        })
    }

    pub fn save(&mut self, conn: &DbConn) -> Result<(), String> {
        match diesel::insert_into(reports::table)
        .values(&*self)
        .execute(&**conn) {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("Error: could not save report data")),
        }
    }

    pub fn find_reports_by_user(user_uuid: String, conn: &DbConn) -> Result<Vec<Reports>, String> {
        use crate::db::schema::reports::dsl::*;

        let results = match reports.filter(user_uuid.eq(user_uuid)).get_results::<Reports>(&** conn) {
            Ok(results) => Ok(results),
            Err(_) => {
                return Err(String::from("No reports yet"));
            }
        };

        return results;
    }
}