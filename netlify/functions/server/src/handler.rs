use std::fmt::Display;

use async_graphql::{
    Request as GraphQlRequest, Response as GraphQlResponse, ServerError as GraphQlError,
};
use http::{HeaderValue, Method, StatusCode};
use http::header::{ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE};
use http::response::Builder;
use lambda_http::{Body, Error, Request, RequestExt, Response};

use crate::{
    errors::{ClientError, ServerError},
    schema::SCHEMA,
};

pub async fn handle_request(request: Request) -> Result<Response<Body>, Error> {
    let query = match *request.method() {
        Method::OPTIONS => return add_cors(Response::builder().status(200)).body(Body::Empty).map_err(Error::from),
        Method::POST => graphql_request_from_post(request),
        Method::GET => graphql_request_from_get(request),
        _ => Err(ClientError::MethodNotAllowed),
    };

    let query = match query {
        Err(e) => {
            return error_response(StatusCode::BAD_REQUEST, graphql_error(e));
        }
        Ok(query) => query,
    };
    let response_body =
        serde_json::to_string(&SCHEMA.execute(query).await).map_err(ServerError::from)?;
    add_cors(Response::builder()
        .status(200)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json")))
        .body(Body::Text(response_body))
        .map_err(ServerError::from)
        .map_err(Error::from)
}

fn graphql_error(message: impl Display) -> String {
    let message = format!("{}", message);
    let response = GraphQlResponse::from_errors(vec![GraphQlError::new(message, None)]);
    serde_json::to_string(&response).expect("Valid response should never fail to serialize")
}

fn error_response(status: StatusCode, body: String) -> Result<Response<Body>, Error> {
    Ok(add_cors(Response::builder().status(status)).body(Body::Text(body))?)
}

fn add_cors(response: Builder) -> Builder {
    response
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("https://studio.apollographql.com"))
        .header(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"))
        .header(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("GET, POST, OPTIONS"))
        .header(ACCESS_CONTROL_ALLOW_CREDENTIALS, HeaderValue::from_static("true"))
}

fn graphql_request_from_post(request: Request) -> Result<GraphQlRequest, ClientError> {
    match request.into_body() {
        Body::Empty => Err(ClientError::EmptyBody),
        Body::Text(text) => {
            serde_json::from_str::<GraphQlRequest>(&text).map_err(ClientError::from)
        }
        Body::Binary(binary) => {
            serde_json::from_slice::<GraphQlRequest>(&binary).map_err(ClientError::from)
        }
    }
}

fn graphql_request_from_get(request: Request) -> Result<GraphQlRequest, ClientError> {
    let params = request.query_string_parameters();
    let query = params.first("query").ok_or(ClientError::MissingQuery)?;
    let mut request = async_graphql::Request::new(query);
    if let Some(operation_name) = params.first("operationName") {
        request = request.operation_name(operation_name)
    }
    if let Some(variables) = params.first("variables") {
        let value = serde_json::from_str(variables).unwrap_or_default();
        let variables = async_graphql::Variables::from_json(value);
        request = request.variables(variables);
    }
    Ok(request)
}