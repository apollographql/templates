mod handler;
mod schema;
mod errors;

use lambda_http::{service_fn, run, Error};
use log::LevelFilter;
use simple_logger::SimpleLogger;

use handler::handle_request;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    let func = service_fn(handle_request);
    run(func).await?;
    Ok(())
}
