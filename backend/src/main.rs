#[macro_use]
extern crate rocket;
use web_project::run;

#[launch]
async fn rocket() -> _ {
    // Removed bcrypt hash on startup - this was slow and unnecessary for production
    // To generate a password hash, use: cargo run --example hash_password
    run().await
}