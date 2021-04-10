use crate::models::{
    BasicInfoOfNoteChangedEventModel, EventModel, NoteCreatedEventModel,
    ParentOfNoteChangedEventModel,
};
use crate::settings::WebServerSettings;
use chrono::Utc;
use std::{convert::Infallible, net::SocketAddr};
use uuid::Uuid;

use warp::{Filter, Rejection, Reply};

pub(crate) struct WebServer {
    pub socket_addr: SocketAddr,
}

impl WebServer {
    pub(crate) fn init(settings: WebServerSettings) -> Self {
        WebServer {
            socket_addr: settings.socket_addr,
        }
    }
    pub(crate) async fn start(self) {
        
    }

    

   
}
