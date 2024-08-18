use serde::{Deserialize, Serialize};
use lambda_http::http::StatusCode;
use http::Method;
use lambda_http::{run, service_fn, tracing, Error, IntoResponse, Request, RequestExt, RequestPayloadExt, Response};

mod url_shortenert;

#[derive(Deserialize)]
struct ShortenUrlRequest {
    url: String
}

#[derive(Serialize)]
pub struct ShortenUrlResponse {
    link_id: String,
}

async fn function_handler(shortener: &url_shortenert::UrlShortener, event: Request) -> Result<impl IntoResponse, Error> {

    match event.method() {
        &Method::POST => {
            println!("received payload: {:?}", event.body());

            if let Some(request) =  event.payload::<ShortenUrlRequest>()? {
                let shortened = shortener.shorten(request.url.as_str());
                let response = ShortenUrlResponse { link_id: shortened };
                generate_api_response(
                    &StatusCode::OK,
                    serde_json::to_string(&response).unwrap(),
                )
            } else {
                generate_api_response(&StatusCode::BAD_REQUEST, format!("could not parse inbound request: {:?}", event.body()))
            }
        },

        &Method::GET => {
            if let Some(link_id) =  event.query_string_parameters().first("link_id") {

                println!("looking for link {link_id}");

                if let Some(url) = shortener.link(link_id) {
                    let response = Response::builder()
                        .status(StatusCode::FOUND)
                        .header("Location", url)
                        .body("".to_string())
                        .map_err(Box::new)?;
                    Ok(response)   
                } else {
                    generate_api_response(&StatusCode::NOT_FOUND, "url not found".to_string())
                }
            } else {
                generate_api_response(&StatusCode::BAD_REQUEST, "missing input link_id".to_string())
            }
        }

        _ => {
            generate_api_response(&StatusCode::BAD_REQUEST, "unsupported method".to_string())
        }
    }
}


pub fn generate_api_response(status: &StatusCode, body: String) -> Result<Response<String>, Error> {
    let response = Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(body)
        .map_err(Box::new)?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    let shortener = url_shortenert::UrlShortener::new();
    run(service_fn(|event| function_handler(&shortener, event))).await
}