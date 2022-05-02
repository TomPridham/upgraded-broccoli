#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::TempFile;
use rocket_dyn_templates::Template;
use serde::Serialize;
use std::collections::HashMap;

type ArbitraryJson = serde_json::Map<String, serde_json::Value>;

#[derive(Responder, Serialize)]
struct Err {
    error: String,
}

#[derive(Responder)]
enum UploadResponses {
    #[response(status = 200, content_type = "json")]
    RawCsvJson(String),
    #[response(status = 500, content_type = "json")]
    UnableToOpenFileJson(Err),
    #[response(status = 500, content_type = "json")]
    UnableToPersistFileJson(Err),
    #[response(status = 400, content_type = "json")]
    BadCsvParseJson(Err),
}

#[derive(FromForm)]
struct Upload<'r> {
    file: TempFile<'r>,
}

const TRY_LATER: &'static str =
    "Please try again later. If this problem persists, please contact support.";

#[post("/upload-csv", data = "<upload>")]
async fn upload_csv(upload: Form<Upload<'_>>) -> UploadResponses {
    let path = match upload.file.path() {
        Some(path) => path,
        None => {
            let error = format!("Unable to temporarily store file. {TRY_LATER}");
            return UploadResponses::UnableToPersistFileJson(Err { error });
        }
    };

    let file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(_) => {
            let error = format!("Unable to acquire handle to file. {TRY_LATER}");
            return UploadResponses::UnableToOpenFileJson(Err { error });
        }
    };

    let mut reader = csv::Reader::from_reader(file);
    let records = reader.deserialize::<ArbitraryJson>();
    let json: Vec<_> = records.flatten().collect();

    let json_str = match serde_json::to_string(&json) {
        Ok(json_str) => json_str,
        Err(_) => {
            return UploadResponses::BadCsvParseJson(Err {
                error: format!("Unable to parse CSV. Please correct your CSV and upload it again"),
            })
        }
    };
    UploadResponses::RawCsvJson(json_str)
}

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, ()> = HashMap::new();
    Template::render("upload_csv", context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, upload_csv])
        .attach(Template::fairing())
}
