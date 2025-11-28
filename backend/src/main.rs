#[macro_use]
extern crate rocket;
use web_project::run;

#[launch]
async fn rocket() -> _ {
    run().await
}