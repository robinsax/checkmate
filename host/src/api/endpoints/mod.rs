use rocket::fairing::AdHoc as AdHocFairing;

mod auth;
mod lobby;
mod game;

pub fn stage() -> AdHocFairing {
    AdHocFairing::on_ignite("endpoints", |rocket| async {
        rocket
            .mount("/v1/auth", auth::routes())
            .mount("/v1/lobby", lobby::routes())
            .mount("/v1/games", game::routes())
    })
}
