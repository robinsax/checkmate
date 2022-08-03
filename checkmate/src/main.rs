use std::io;
use std::env;

use tokio;
use fern;
use log::info;

use checkmate::runtimes;

fn configure_logger() -> Result<(), fern::InitError> {
    let dest_file = match env::var("CHECKMATE_HOST_LOG_FILE") {
        Ok(f) => f,
        Err(_) => "checkmate_host_runtime.log".to_string()
    };

    fern::Dispatch::new()
        .chain(fern::log_file(dest_file)?)
        .chain(io::stdout())
        .apply()?;
    Ok(())
}

fn main() {
    configure_logger().unwrap();
    info!("booting checkmate host");

    let runtime = tokio::runtime::Runtime::new().unwrap();

    let server = runtimes::create_api();

    runtime.block_on(server.launch());
}