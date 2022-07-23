use std::{ io, env };

use rocket;
use fern::{ Dispatch, InitError, log_file };
use log::{ info };

mod server;

use server::{ create_server };

fn configure_logger() -> Result<(), InitError> {
    let dest_file = match env::var("CHECKMATE_HOST_LOG_FILE") {
        Ok(val) => val,
        _ => "checkmate_host_runtime.log".to_string(),
    };

    Dispatch::new()
        .chain(log_file(dest_file)?)
        .chain(io::stdout())
        .apply()?;
    Ok(())
}

#[rocket::main]
async fn main() {
    configure_logger().unwrap();

    info!("booting checkmate host");
    create_server().launch().await;
}
