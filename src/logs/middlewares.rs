use actix_web::dev::{ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use actix_web::dev::Service;
use futures::future::{ok, Ready};
use crate::logs::repository::add_log_entry;

pub struct RequestLogger;

impl<S> Transform<S, ServiceRequest> for RequestLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = RequestLoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestLoggerMiddleware { service })
    }
}

pub struct RequestLoggerMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for RequestLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {


        let method = req.method().to_string();
        let path = req.path().to_string();



        // Capturar IP
        let connection_info = req.connection_info().clone();
        let ip = connection_info.realip_remote_addr().map(|ip| ip.to_string()).unwrap_or("unknown".to_string());


        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            add_log_entry(
                "INFO",
                "Request processed",
                &path,
                Some(method),
                Some(res.status().as_u16()),
                Some(ip),
            );

            Ok(res)
        })
    }
}