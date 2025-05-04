use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use actix_web::body::BoxBody;
use futures_util::future::LocalBoxFuture;

// Custom logging middleware
pub struct RequestLogger;

impl<S> actix_web::dev::Transform<S, ServiceRequest> for RequestLogger
where
    S: actix_web::dev::Service<
        ServiceRequest,
        Response = ServiceResponse<BoxBody>,
        Error = Error,
    > + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = RequestLoggerMiddleware<S>;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        Box::pin(async move { Ok(RequestLoggerMiddleware { service }) })
    }
}

pub struct RequestLoggerMiddleware<S> {
    service: S,
}

impl<S> actix_web::dev::Service<ServiceRequest> for RequestLoggerMiddleware<S>
where
    S: actix_web::dev::Service<
        ServiceRequest,
        Response = ServiceResponse<BoxBody>,
        Error = Error,
    > + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().to_string();
        let path = req.path().to_string();

        tracing::info!(
            method = method,
            path = path,
            "Incoming request",
        );

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            tracing::info!(
                status = res.status().as_u16(),
                method = method,
                path = path,
                "Outgoing response",
            );
            Ok(res)
        })
    }
}
