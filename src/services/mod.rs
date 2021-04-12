use crate::settings::Settings;

mod web_server;
mod kafka_service;
mod mongodb_service;
use tokio::join;
use self::{kafka_service::KafkaService, web_server::WebServer};
use self::mongodb_service::MongodbService;

pub(crate) async fn run(settings:Settings) {

    
    let mongodb_service = MongodbService::init(settings.mongodb_settings).await;
    let kafka_service = KafkaService::init(settings.kafka_settings, mongodb_service.clone());
    let web_server = WebServer::init(settings.web_server_settings,mongodb_service);
    //web_server.start().await;
    join!( web_server.start(), kafka_service.start_polling() ); 
    
}