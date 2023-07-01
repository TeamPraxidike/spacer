use rocket::{
    async_trait,
    fairing::{Fairing, Info, Kind},
    Build,
};

pub struct ApiFairing;

#[async_trait]
impl Fairing for ApiFairing {
    fn info(&self) -> Info {
        Info {
            name: "API fairing",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: rocket::Rocket<Build>) -> rocket::fairing::Result {
        Ok(rocket.mount("/", rocket::routes![new_project]))
    }
}

#[rocket::post("/projects/new")]
async fn new_project() -> &'static str {
    ""
}
