use serde::Deserialize;
use lambda_http::http::{StatusCode};
use lambda_http::{run, service_fn, tracing,  Error, IntoResponse, Request, RequestExt, RequestPayloadExt, Response};

mod url_shortener;
mod url_shortener_concurrent;


#[derive(Deserialize)]
struct ShortenUrlRequest {
    url: String
}


async fn function_handler(shortener: &url_shortener_concurrent::UrlShortenerConcurrent, event: Request) -> Result<impl IntoResponse, Error> {

    match event.method().as_str() {
        "POST" => {
            if let Some(request) =  event.payload::<ShortenUrlRequest>()? {
                let shortened = shortener.shorten(request.url.as_str());
                generate_api_response(
                    200,
                    &serde_json::to_string(&shortened).unwrap(),
                )
            } else {
                generate_api_response(400, "could not parse inbound request")
            }
        },

        "GET" => {
            if let Some(link_id) =  event.query_string_parameters().first("link_id") {
                if let Some(url) = shortener.link(link_id) {
                    let response = Response::builder()
                        .status(StatusCode::from_u16(302).unwrap())
                        .header("Location", url)
                        .body("".to_string())
                        .map_err(Box::new)?;

                    Ok(response)   
                } else {
                    generate_api_response(404, "url not found")
                }
            } else {
                generate_api_response(400, "missing input url")
            }
        }

        _ => {
            generate_api_response(400, "invalid request")
        }

    }
}


pub fn generate_api_response(status: u16, body: &str) -> Result<Response<String>, Error> {
    let response = Response::builder()
        .status(StatusCode::from_u16(status).unwrap())
        .header("content-type", "application/json")
        .body(body.to_string())
        .map_err(Box::new)?;
    Ok(response)
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    let shortener = url_shortener_concurrent::UrlShortenerConcurrent::new();
    run(service_fn(|event| function_handler(&shortener, event))).await
}