use rocket::fairing::AdHoc as AdHocFairing;

mod auth;
//mod games;

pub use auth::{User, AuthRegistryState, AuthToken};
//pub use games::GameHostState;

pub fn stage() -> AdHocFairing {
    AdHocFairing::on_ignite("endpoints", |rocket| async {
        rocket
            .manage(auth::AuthRegistry::new().into_state())
            //.manage(games::GameHost::new().into_state())
    })
}
