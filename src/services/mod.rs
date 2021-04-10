use crate::settings::Settings;

mod web_server;
mod kafka_service;
mod mongodb_service;

use self::kafka_service::KafkaService;
use self::mongodb_service::MongodbService;

pub(crate) async fn run(settings:Settings) {

    
    let mongodb_service = MongodbService::init(settings.mongodb_settings).await;
    let kafka_service = KafkaService::init(settings.kafka_settings, mongodb_service);

    kafka_service.start_polling().await;
    
}