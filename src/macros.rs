// Macro to wrap handlers and convert NasaApiError to worker::Error
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