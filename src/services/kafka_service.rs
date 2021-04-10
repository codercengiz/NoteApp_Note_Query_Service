extern crate env_logger;
extern crate kafka;

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;

use crate::models::EventModel;
use crate::settings::KafkaSettings;

use super::mongodb_service::MongodbService;

pub(crate) struct KafkaService {
    brokers: String,
    topics: String,
    mongodb_service:MongodbService,
}
impl KafkaService {
    pub fn init(settings: KafkaSettings,mongodb_service:MongodbService) -> Self {
        KafkaService {
            brokers: settings.broker,
            topics: settings.consumer_topics[0].to_string(),
            mongodb_service,
        }
    }

    pub(crate) async fn start_polling(&self) {
        let mut consumer = Consumer::from_hosts(vec![self.brokers.to_owned()])
            .with_topic(self.topics.to_owned())
            .with_group("noteapp".to_string())
            .with_fallback_offset(FetchOffset::Earliest)
            .with_offset_storage(GroupOffsetStorage::Kafka)
            .create().unwrap();

        log::debug!("Starting kafka consumer");

        loop {
            let ms = consumer.poll().unwrap();
            if ms.is_empty() {
                continue;
            }

            for messages in ms.iter() {
                for message in messages.messages() {
                    match serde_json::from_slice::<EventModel>(message.value) {
                        Ok(event) => {
                            println!("{:?}", &event);
                            self.apply_event(event).await;
                        }
                        Err(error @ serde_json::Error { .. }) if error.is_eof() => {}
                        Err(_) => {}
                    }

                    if let Err(_) = consumer.consume_message(
                        messages.topic(),
                        messages.partition(),
                        message.offset,
                    ) {
                        log::error!("Could not mark message as consumed");
                    }
                }
            }
            consumer.commit_consumed();
        }
    }

    pub(crate) async fn apply_event(&self, event: EventModel) {


        match event {
            EventModel::NoteCreatedEventModel(note_created_event_model) => {}
            EventModel::ParentOfNoteChangedEventModel(_) => {}
            EventModel::BasicInfoOfNoteChangedEventModel(_) => {}
        }
    }
}
