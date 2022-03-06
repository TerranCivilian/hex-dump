#[macro_use] extern crate rocket;

use std::fmt::Write;
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

fn write_hexdump(bytes: &Vec<u8>) -> String {
    let mut hexdump = String::new();
    for (i, c) in bytes.iter().enumerate() {
        if i % 16 == 0 {
            write!(hexdump, "\n{:#08x}: ", i).unwrap();
        }
        write!(hexdump, "{:02x} ", c).unwrap();
    }
    hexdump
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![hex])
}