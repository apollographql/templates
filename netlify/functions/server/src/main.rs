mod errors;
mod handler;
mod schema;

use handler::handle_request;
use lambda_http::{run, service_fn, Error};
use log::LevelFilter;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let func = service_fn(handle_request);
    run(func).await?;
    Ok(())
}
