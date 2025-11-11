#[macro_use]
extern crate rocket;
use web_project::run;

#[launch]
async fn rocket() -> _ {
    println!("admin token: {}", bcrypt::hash("admin", 12).unwrap());
    run().await
}