use colored::Colorize;
use http::{method::Method, Request, Response};
use pin_project::pin_project;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::Instant;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct LoggerLayer {}

impl LoggerLayer {
    pub fn new() -> Self {
        LoggerLayer {}
    }
}

impl<S> Layer<S> for LoggerLayer {
    type Service = Logger<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Logger::new(inner)
    }
}

#[derive(Clone)]
pub struct Logger<S> {
    inner: S,
}

impl<S> Logger<S> {
    pub fn new(inner: S) -> Self {
        Logger { inner }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for Logger<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
    ResBody: Default,
    <S as Service<Request<ReqBody>>>::Error: Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = LoggerFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let method = req.method().clone();
        let path = req.uri().path().to_string();

        LoggerFuture {
            future: self.inner.call(req),
            method,
            path,
            start: Instant::now(),
        }
    }
}

#[pin_project]
pub struct LoggerFuture<F> {
    #[pin]
    future: F,
    method: Method,
    path: String,
    start: Instant,
}

impl<F, B, E> Future for LoggerFuture<F>
where
    F: Future<Output = Result<Response<B>, E>>,
    B: Default,
    E: Debug,
{
    type Output = Result<Response<B>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let res: F::Output = match this.future.poll(cx) {
            Poll::Ready(res) => res,
            Poll::Pending => return Poll::Pending,
        };

        if this.path.starts_with("/__tuono/data") {
            return Poll::Ready(res);
        }

        let status_code = res.as_ref().unwrap().status();

        println!(
            "  {} {} {} in {}ms",
            this.method,
            this.path,
            status_code.as_str().green(),
            this.start.elapsed().as_millis()
        );

        Poll::Ready(res)
    }
}
