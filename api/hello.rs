use farcaster_frames_template::{get_character, APIError, FrameActionPayload};
use vercel_runtime::{
    http::bad_request, process_request, process_response, run_service, service_fn, Body,
    Error, Request, RequestPayloadExt, Response, ServiceBuilder, StatusCode,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber
        ::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        .init();

    // This allows to extend the tower service with more layers
    let handler = ServiceBuilder::new()
        .map_request(process_request)
        .map_response(process_response)
        .service(service_fn(handler));

    run_service(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    tracing::info!("Handler init, method: {}", req.method());

    match req.method().as_str() {
        "GET" => handle_get_request(req).await,
        "POST" => handle_post_request(req).await,
        _ => Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::from("Method Not Allowed"))?),
    }
}

pub async fn handle_get_request(req: Request) -> Result<Response<Body>, Error> {
    tracing::info!("Get Handler init");

    let frame_image =
        "https://upload.wikimedia.org/wikipedia/commons/6/6c/Star_Wars_Logo.svg";
    let frame_post_url = req.uri();
    let html_content = format!(
        r#"<!DOCTYPE html>
            <html lang="en">
            <head>
                <meta property="og:image" content="{0}" /> 
                <meta property="fc:frame" content="vNext" />
                <meta property="fc:frame:post_url" content="{1}" />
                <meta property="fc:frame:image" content="{0}" />
                <meta property="fc:frame:button:1" content="What Star Wars guy am I?" />
                <title>Farcaster Frames Template</title>
            </head>
            <body>
                <h1>What Star Wars guy am I?</h1>
            </body>
            </html>"#,
        frame_image, frame_post_url
    );

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(html_content.into())?)
}

pub async fn handle_post_request(req: Request) -> Result<Response<Body>, Error> {
    tracing::info!("Post Handler init");

    let payload = req.payload::<FrameActionPayload>();
    match payload {
        Err(err) => {
            tracing::info!("Invalid payload {}", err);
            bad_request(APIError {
                message: "Invalid payload",
                code: "invalid_payload",
            })
        }
        Ok(None) => {
            tracing::info!("No payload");
            bad_request(APIError {
                message: "No payload",
                code: "no_payload",
            })
        }
        Ok(Some(payload)) => {
            tracing::info!("payload {}", payload);

            let character = get_character();
            let frame_image = format!(
                "https://placehold.co/600x400/black/yellow?text={}",
                character
            );
            let html_content = format!(
                r#"<!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta property="og:image" content="{0}" /> 
                    <meta property="fc:frame" content="vNext" />
                    <meta property="fc:frame:image" content="{0}" />
                    <title>Farcaster Frames Template</title>
                </head>
                </html>"#,
                frame_image
            );

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(html_content.into())?)
        }
    }
}
