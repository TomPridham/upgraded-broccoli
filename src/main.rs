#[macro_use]
extern crate rocket;

use rocket::data::{Data, ToByteUnit};
use serde::Serialize;

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
    UnableToConvertStreamJson(Err),
    #[response(status = 400, content_type = "json")]
    BadCsvParseJson(Err),
    #[response(status = 400, content_type = "json")]
    FileTooLargeJson(Err),
}

#[post("/upload-csv", data = "<data>")]
async fn upload_csv(data: Data<'_>) -> UploadResponses {
    let stream = data.open(5.mebibytes());
    let s = match stream.into_string().await{
        Ok(s) => s,
        Err(_) => return UploadResponses::UnableToConvertStreamJson(Err{error:format!("Unable to open file. Please try again later. If this problem persists, please contact support")})
    };

    if !s.is_complete() {
        return UploadResponses::FileTooLargeJson(Err { error: format!("This api only supports file uploads up to 5MB. Please reduce the file size and try again") });
    }

    // skipping headers
    let body = s.lines().skip(3).collect::<Vec<_>>();
    // skipping trailing value
    let (_, body) = body.split_last().unwrap();
    let body = body.join("\n");

    let mut reader = csv::Reader::from_reader(body.as_bytes());
    let records = reader.deserialize::<ArbitraryJson>();
    let json: Vec<_> = records.flatten().collect();

    let json_str = match serde_json::to_string(&json) {
        Ok(json_str) => json_str,
        Err(_) => {
            return UploadResponses::BadCsvParseJson(Err {
                error: format!("Unable to parse csv"),
            })
        }
    };
    UploadResponses::RawCsvJson(json_str)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![upload_csv])
}
