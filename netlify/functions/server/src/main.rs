use aws_lambda_events::encodings::Body;
use http::Response;
use lambda_http::{service_fn, run, Request, Error};
use log::LevelFilter;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    let func = service_fn(my_handler);
    run(func).await?;
    Ok(())
}

pub(crate) async fn my_handler(_request: Request) -> http::Result<Response<Body>> {
    Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body(format!("Hello! I'm updated now.").into())
}