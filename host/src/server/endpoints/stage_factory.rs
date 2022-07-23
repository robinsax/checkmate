use std::sync::{ Arc, Mutex };

use rocket::fairing::{ AdHoc as AdHocFairing };

use super::auth::{ AuthRegistry, auth_routes };

pub fn stage() -> AdHocFairing {
    AdHocFairing::on_ignite("endpoints", |rocket| async {
        rocket
            .mount("/v1/auth", auth_routes())
            .manage(Arc::new(Mutex::new(AuthRegistry::new())))
    })
}
