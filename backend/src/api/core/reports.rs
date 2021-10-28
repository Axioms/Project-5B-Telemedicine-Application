use chrono::Utc;
use rocket_contrib::json::Json;

use crate::{

    db::{models::*, DbConn},
    api::EmptyResult,
    api::StringResult,
    api::JsonResult,
    utils::auth::{Headers},
};

pub fn routes() -> Vec<rocket::Route> {
    routes![
        list_all_reports,
        add_report,
    ]
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct TestData {
    Email: String
}

#[post("/reports/list/all")]
fn list_all_reports(headers: Headers, conn: DbConn) -> JsonResult {
    let uuid = headers.user.uuid;
    let reports = Reports::find_reports_by_user(uuid, &conn);
    return Ok(Json(json!({"reports": reports.unwrap()})))
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct ReportData {
    patient_uuid: String,
    report: String
}
#[post("/reports/add", data = "<data>")]
fn add_report(data: Json<ReportData>, headers: Headers, conn: DbConn) {
    let report_data = String::from(&data.report);
    let patient_uuid = String::from(&data.patient_uuid);
    let mut report = Reports::new(headers.user.uuid, patient_uuid, report_data);
    report.save(&conn);
}