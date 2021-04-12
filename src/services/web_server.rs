use super::mongodb_service::{MongodbService, MongodbServiceError};
use crate::models::NoteModel;
use crate::settings::WebServerSettings;

use futures::TryFutureExt;
use std::{convert::Infallible, net::SocketAddr};
use thiserror::Error;
use warp::{hyper::StatusCode, Filter, Rejection, Reply};

pub(crate) struct WebServer {
    pub socket_addr: SocketAddr,
    pub mongodb_service: MongodbService,
}

impl WebServer {
    pub(crate) fn init(settings: WebServerSettings, mongodb_service: MongodbService) -> Self {
        WebServer {
            socket_addr: settings.socket_addr,
            mongodb_service,
        }
    }
    pub(crate) async fn start(self) {
        let routes = self
            .get_all_notes_route()
            .with(
                warp::cors()
                    .allow_any_origin()
                    .allow_methods(vec!["POST", "GET"]),
            )
            .with(warp::log("webserver"))
            .recover(Self::handle_rejection);
        warp::serve(routes).run(self.socket_addr).await;
    }
    fn get_all_notes_route(&self) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path::param::<String>()
            .and(warp::path::end())
            .and(warp::get())
            .and(Self::with_mongodb_service(self.mongodb_service.clone()))
            .and_then(Self::get_all_notes_handler)
    }

    async fn get_all_notes_handler(
        user_id: String,
        mongodb_service: MongodbService,
    ) -> Result<impl warp::Reply, Rejection> {
        Self::get_all_notes(user_id, mongodb_service)
            .map_ok(|ref note| warp::reply::json(note))
            .map_err(warp::reject::custom)
            .await
    }
    async fn get_all_notes(
        user_id: String,
        mongodb_service: MongodbService,
    ) -> Result<Vec<NoteModel>, WebServerError> {
        debug!("Get notes for user : {:?}", user_id);

        mongodb_service
            .get_all_notes(user_id)
            .await
            .map_err(WebServerError::MongodbServiceError)
    }
    fn with_mongodb_service(
        mongodb_service: MongodbService,
    ) -> impl Filter<Extract = (MongodbService,), Error = Infallible> + Clone {
        warp::any().map(move || mongodb_service.clone())
    }
    async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
        let code;
        let message;

        if err.is_not_found() {
            error!("{:?}", err);
            code = StatusCode::NOT_FOUND;
            message = "Not Found";
        } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
            error!("{:?}", err);
            code = StatusCode::METHOD_NOT_ALLOWED;
            message = "Method Not Allowed";
        } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
            error!("{:?}", err);
            code = StatusCode::BAD_REQUEST;
            message = "Invalid Body";
        } else if let Some(_) = err.find::<WebServerError>() {
            error!("{:?}", err);
            code = StatusCode::INTERNAL_SERVER_ERROR;
            message = "Application Error";
        } else {
            error!("unhandled error: {:?}", err);
            code = StatusCode::INTERNAL_SERVER_ERROR;
            message = "Internal Server Error";
        }

        Ok(warp::reply::with_status(message, code))
    }
}

#[derive(Debug, Error)]
pub(crate) enum WebServerError {
    #[error(transparent)]
    MongodbServiceError(#[from] MongodbServiceError),
}

impl warp::reject::Reject for WebServerError {}
