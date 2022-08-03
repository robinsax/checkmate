use rocket;

mod cors;
mod catchers;
mod responses;
mod state;
mod endpoints;

pub fn create_api() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(cors::stage())
        .attach(state::stage())
        .attach(endpoints::stage())
        .attach(catchers::stage())
}
