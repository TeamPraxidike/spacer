use rocket::Rocket;
use spacer::ApiFairing;

#[rocket::launch]
fn rocket() -> _ {
    Rocket::build()
        .attach(ApiFairing)
}