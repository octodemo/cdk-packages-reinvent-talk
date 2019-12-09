use std::error::Error;

use lambda_http::{lambda, Body, Request, RequestExt, Response};
use lambda_runtime::{error::HandlerError, Context};
use log::{self, error};
use serde_derive::{Deserialize, Serialize};
use simple_error::bail;
use simple_logger;

#[derive(Deserialize, Default)]
struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
}

#[derive(Serialize)]
struct CustomOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(hello_handler);

    Ok(())
}

fn hello_handler(request: Request, c: Context) -> Result<Response<Body>, HandlerError> {
    let ce: CustomEvent = request
        .payload()
        .unwrap_or_else(|_parse_err| None)
        .unwrap_or_default();

    if ce.first_name == "" {
        error!("Empty first name in request {}", c.aws_request_id);
        bail!("Empty first name");
    }

    Ok(Response::new(format!("Hello {}!\n", ce.first_name).into()))
}
