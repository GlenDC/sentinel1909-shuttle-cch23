// src/lib/router.rs

// dependencies
use crate::routes::root::root;
use day1_endpoints::{calibrate_packet_ids, calibrate_sled_ids};
use day4_endpoints::get_strength;
use hyper::{Body, Request, Response, StatusCode};
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::Service;

// a struct type to represent the app router
#[derive(Clone)]
pub struct Router;

// implementation block for the Router type; instantiates and provides a general  internal server error
impl Router {
    pub fn create() -> Self {
        Self
    }

    fn internal_server_error(&self) -> Response<Body> {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Internal Server Error"))
            .unwrap()
    }

    fn bad_request(&self) -> Response<Body> {
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Bad Request"))
            .unwrap()
    }
}

// implement the Tower Service trait for the Router type

impl Service<Request<Body>> for Router {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + Sync>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        // get the url from the request, convert it to a string
        let url = request.uri().to_string();

        // get the method from the request, convert it to a string
        let method = request.method().to_string();

        // get the path portion from the request url
        let path = request.uri().path();

        // split the path into segments, parse the segments into i32 values
        let path_segments: Vec<Result<i32, _>> = path
            .split('/')
            .map(|segment| segment.parse::<i32>())
            .collect();

        // filter out any segments that are not Ok, collect the remaining values into a vector
        let values: Vec<i32> = path_segments
            .into_iter()
            .filter_map(|path_segment| path_segment.ok())
            .collect();

        // create a dynamic path based on the values in the vector
        if values.len() > 1 {
            let _calibrate_url = values
                .iter()
                .map(|&value| value.to_string())
                .collect::<Vec<String>>()
                .join("/");
        }

        // match the url against the routes
        let response = match url.as_str() {
            "/" if method == "GET" => match root() {
                Ok(resp) => resp,
                Err(_) => self.internal_server_error(),
            },
            "/-1/error" => self.internal_server_error(),
            _calibrate_url if method == "GET" && values.len() == 3 => {
                match calibrate_packet_ids(values) {
                    Ok(resp) => resp,
                    Err(_) => self.internal_server_error(),
                }
            }
            _calibrate_url if method == "GET" && values.len() < 21 => {
                match calibrate_sled_ids(values) {
                    Ok(resp) => resp,
                    Err(_) => self.bad_request(),
                }
            }
            "/4/strength" if method == "POST" => match get_strength(request) {
                Ok(resp) => resp,
                Err(_) => self.bad_request(),
            },
            _ => self.bad_request(),
        };

        let fut = async { Ok(response) };

        Box::pin(fut)
    }
}
