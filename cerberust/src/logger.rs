use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, TraceLayer};
use tracing::Level;

pub fn logger(
) -> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>>
{
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
}
