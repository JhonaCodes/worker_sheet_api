use std::collections::HashMap;
use actix_web::dev::{ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use actix_rt::time::Instant;
use actix_web::dev::Service;
use actix_web::http::header;
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
        let start_time = Instant::now();
        let method = req.method().to_string();
        let path = req.path().to_string();

        // Capturar IP
        let ip = req.connection_info().realip_remote_addr()
            .map(|ip| ip.to_string());

        // Capturar headers importantes
        let mut important_headers = HashMap::new();
        if let Some(user_agent) = req.headers().get(header::USER_AGENT) {
            important_headers.insert("user_agent".to_string(), user_agent.to_str().unwrap_or("").to_string());
        }
        if let Some(auth) = req.headers().get(header::AUTHORIZATION) {
            important_headers.insert("authorization".to_string(), "**REDACTED**".to_string());
        }
        if let Some(content_type) = req.headers().get(header::CONTENT_TYPE) {
            important_headers.insert("content_type".to_string(), content_type.to_str().unwrap_or("").to_string());
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await;
            let duration = start_time.elapsed().as_millis();

            match res {
                Ok(response) => {
                    add_log_entry(
                        "INFO",
                        "Request processed",
                        &path,
                        Some(method),
                        Some(response.status().as_u16()),
                        ip,
                        Some(duration),
                        important_headers.get("user_agent").cloned(),
                        None,
                        Some(important_headers)
                    );
                    Ok(response)
                }
                Err(err) => {
                    add_log_entry(
                        "ERROR",
                        "Request failed",
                        &path,
                        Some(method),
                        None,
                        ip,
                        Some(duration),
                        important_headers.get("user_agent").cloned(),
                        Some(err.to_string()),
                        Some(important_headers)
                    );
                    Err(err)
                }
            }
        })
    }

}