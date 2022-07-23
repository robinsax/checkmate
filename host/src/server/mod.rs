use rocket::{ Rocket, Build, build as rocket_build };

mod cors;
mod catchers;
mod endpoints;
mod responses;

pub fn create_server() -> Rocket<Build> {
    rocket_build()
        .attach(cors::stage())
        .attach(endpoints::stage())
        .attach(catchers::stage())
}
