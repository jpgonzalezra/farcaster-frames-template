use vercel_runtime::{
    process_request, process_response, run_service, service_fn, Body, Error, Request,
    Response, ServiceBuilder, StatusCode,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber
        ::fmt()
        .with_max_level(tracing::Level::ERROR)
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

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    tracing::info!("Handler init");

    let frame_image =
        "https://upload.wikimedia.org/wikipedia/commons/6/6c/Star_Wars_Logo.svg";

    let html_content = format!(
        r#"<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta property="og:image" content="{}" /> 
            <meta property="fc:frame" content="vNext" />
            <meta property="fc:frame:image" content="{}" />
            <meta property="fc:frame:button:1" content="What Star Wars guy am I?" />
            <title>Farcaster Frames</title>
        </head>
        <body>
            <h1>What Star Wars guy am I?</h1>
        </body>
        </html>"#,
        frame_image, frame_image
    );

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(html_content.into())?)
}
