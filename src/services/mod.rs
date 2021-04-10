use crate::settings::Settings;

mod web_server;
mod kafka_service;
mod mongodb_service;

use self::kafka_service::KafkaService;

pub(crate) async fn run(settings:Settings) {

    let kafka_service = KafkaService::init(settings.kafka_settings);

    kafka_service.consume_messages().await;
    
}