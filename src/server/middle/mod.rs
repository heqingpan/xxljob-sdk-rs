use std::future::{ready, Ready};
use std::sync::Arc;

use crate::common::model::XxlApiResult;
use crate::common::share_data::ShareData;
use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;

#[derive(Clone)]
pub struct CheckMiddle {
    share_data: Arc<ShareData>,
}

impl CheckMiddle {
    pub fn new(share_data: Arc<ShareData>) -> Self {
        Self { share_data }
    }
}

impl<S, B> Transform<S, ServiceRequest> for CheckMiddle
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckMiddleware {
            service: Arc::new(service),
            share_data: self.share_data.clone(),
        }))
    }
}

#[derive(Clone)]
pub struct CheckMiddleware<S> {
    service: Arc<S>,
    share_data: Arc<ShareData>,
}

impl<S, B> Service<ServiceRequest> for CheckMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let token = if let Some(v) = request.headers().get("XXL-JOB-ACCESS-TOKEN") {
            v.to_str().unwrap_or_default().to_owned()
        } else {
            "".to_owned()
        };
        let is_check_success = self.share_data.client_config.access_token.is_empty()
            || self.share_data.client_config.access_token.as_str() == token.as_str();

        let service = self.service.clone();
        Box::pin(async move {
            if is_check_success {
                let res = service.call(request);
                // forwarded responses map to "left" body
                res.await.map(ServiceResponse::map_into_left_body)
            } else {
                //没有登录
                let response = HttpResponse::Ok()
                    .json(XxlApiResult::<()>::fail(Some(
                        "access-token is error".to_string(),
                    )))
                    .map_into_right_body();
                let (http_request, _pl) = request.into_parts();
                let res = ServiceResponse::new(http_request, response);
                Ok(res)
            }
        })
    }
}
