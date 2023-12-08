// day5/src/lib.rs

// 2023 Shuttle Christmas Code Hunt - Day 5 Challenge Endpoints

// dependencies
use errors::ApiError;
use hyper::{Body, Response, StatusCode};

pub fn grinch() -> Result<Response<Body>, ApiError> {
    // create the response body
    let response_body = "You're a mean one, Mr. Grinch!";

    // return the reindeer strength
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(response_body))?)
}
