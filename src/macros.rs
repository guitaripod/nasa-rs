//! Utility macros for the NASA API Proxy Worker.

/// Wraps an async handler function to automatically convert NasaApiError to HTTP responses.
/// 
/// This macro simplifies error handling in route handlers by automatically converting
/// any `NasaApiError` returned by the handler into a proper HTTP response with the
/// appropriate status code and error message.
/// 
/// # Example
/// 
/// ```ignore
/// wrap_handler!(async |req, ctx| {
///     // Handler logic that returns Result<Response, NasaApiError>
///     Ok(Response::ok("Success")?)
/// })
/// ```
#[macro_export]
macro_rules! wrap_handler {
    ($handler:expr) => {
        |req, ctx| async move {
            match $handler(req, ctx).await {
                Ok(response) => Ok(response),
                Err(e) => Ok(e.to_response()),
            }
        }
    };
}