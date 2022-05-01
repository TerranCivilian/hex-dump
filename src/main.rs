#[macro_use] extern crate rocket;

use rocket::data::{Data, ToByteUnit};

#[get("/")]
fn index() -> &'static str {
    "
    hexdump program...
    POST files to /
    "
}

#[post("/", data = "<file>")]
async fn hex(file: Data<'_>) -> std::io::Result<String> {
    let bytes = file.open(2.mebibytes()).into_bytes().await?.value;
    Ok(write_hexdump(&bytes))
}

fn write_hexdump(bytes: &[u8]) -> String {
    bytes.iter().enumerate().fold("".to_string(), |acc, (i, &byte)| {
        let mut line = if i % 16 == 0 { format!("\n{:#08x}: ", i) } else { "".to_string() };
        line += &format!("{:02x} ", byte);
        acc + &line
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![hex])
}
