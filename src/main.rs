#[macro_use]
extern crate rocket;

use rocket::data::{Data, ToByteUnit};
use rocket::fs::TempFile;
use rocket::futures::StreamExt;
use rocket::tokio;

#[get("/")]
fn index() -> &'static str {
    "hey"
}

#[post("/upload-csv", data = "<data>")]
async fn upload_csv(data: Data<'_>) -> std::io::Result<()> {
    let stream = data.open(5.mebibytes());
    let s = stream.into_string().await?;

    let mut body = s.lines().skip(3).collect::<Vec<_>>();
    body.pop();
    let bytes = body.join("\n");
    println!("b: {bytes:?}");
    let mut reader = csv_async::AsyncReader::from_reader(bytes.as_bytes());
    let mut records = reader.records();
    while let Some(record) = records.next().await {
        println!("a: {record:?}");
    }
    Ok(())
}

#[launch]
fn rocket() -> _ {
    println!("Hello, world!");
    rocket::build().mount("/", routes![index, upload_csv])
}
