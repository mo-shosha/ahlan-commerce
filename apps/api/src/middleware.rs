
use axum::{
    extract::Request,
    http::header,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

pub async fn request_id_middleware(mut request: Request,next: Next) -> Response {
    
    let request_id = Uuid::now_v7().to_string();

    // Store the request id to make handlers can access it.
    request.extensions_mut().insert(request_id.clone());

    let mut response = next.run(request).await;

    // make the same request id in the response header.
    response.headers_mut().insert(
        header::HeaderName::from_static("x-request-id"),
        header::HeaderValue::from_str(&request_id)
            .expect("UUID is a valid header value"),
    );

    response
}
