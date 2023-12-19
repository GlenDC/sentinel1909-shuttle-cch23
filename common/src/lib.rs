// common/src/lib.rs

// dependencies
use hyper::{
    body::Body,
    header::{HeaderValue, CONTENT_TYPE},
    Request, Response, StatusCode,
};
use std::convert::Infallible;

// type aliases
pub type WebRequest = Request<Body>;
pub type WebResponse = Response<Body>;

// IntoWebResponse trait
pub trait IntoWebResponse {
    fn into_web_response(self) -> WebResponse;
}

impl IntoWebResponse for WebResponse {
    fn into_web_response(self) -> WebResponse {
        self
    }
}

impl IntoWebResponse for Infallible {
    fn into_web_response(self) -> WebResponse {
        panic!("Something really bad just happened...")
    }
}

impl IntoWebResponse for StatusCode {
    fn into_web_response(self) -> WebResponse {
        Response::builder()
            .status(self)
            .body(Body::empty())
            .expect("the StatusCode web response to be built")
    }
}

impl IntoWebResponse for &'static str {
    fn into_web_response(self) -> WebResponse {
        Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, HeaderValue::from_static("text/plain"))
            .body(Body::from(self))
            .expect("the &'static str web response to be built")
    }
}

impl IntoWebResponse for String {
    fn into_web_response(self) -> WebResponse {
        Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, HeaderValue::from_static("text/plain"))
            .body(Body::from(self))
            .expect("the string web response to be built")
    }
}

impl IntoWebResponse for Vec<u8> {
    fn into_web_response(self) -> WebResponse {
        Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, HeaderValue::from_static("image/png"))
            .body(Body::from(self))
            .expect("the Vec<u8> response to be built")
    }
}

impl IntoWebResponse for std::io::Error {
    fn into_web_response(self) -> WebResponse {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(CONTENT_TYPE, HeaderValue::from_static("text/plain"))
            .body(Body::from(self.to_string()))
            .expect("the IO error web response to be built")
    }
}

impl IntoWebResponse for Box<dyn std::error::Error> {
    fn into_web_response(self) -> WebResponse {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(CONTENT_TYPE, HeaderValue::from_static("text/plain"))
            .body(Body::from(self.to_string()))
            .expect("the Boxed error web response to be built")
    }
}
