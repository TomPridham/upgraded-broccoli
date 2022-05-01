#[macro_use]
extern crate rocket;

use rocket::data::{Data, ToByteUnit};
use rocket::tokio;

#[get("/")]
fn index() -> &'static str {
    "hey"
}

#[post("/upload-csv", data = "<data>")]
async fn upload_csv(data: Data<'_>) -> std::io::Result<()> {
    data.open(1024.kibibytes())
        .stream_to(tokio::io::stdout())
        .await?;
    Ok(())
}

#[launch]
fn rocket() -> _ {
    println!("Hello, world!");
    rocket::build().mount("/", routes![index, upload_csv])
}
