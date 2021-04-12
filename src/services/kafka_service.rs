extern crate env_logger;
extern crate kafka;
use futures::stream::StreamExt;

use rdkafka::{consumer::{CommitMode, Consumer, StreamConsumer}, message::FromBytes};
use rdkafka::message::Message;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::AsyncRuntime;
use rdkafka::{
    config::{ClientConfig, RDKafkaLogLevel},
    consumer::{ConsumerContext, Rebalance},
    error::KafkaResult,
    ClientContext, TopicPartitionList,
};
use std::{future::Future, str::Utf8Error};

use crate::models::EventModel;
use crate::settings::KafkaSettings;

use super::mongodb_service::MongodbService;

struct CustomContext;
type LoggingConsumer = StreamConsumer<CustomContext>;
impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}
pub(crate) struct KafkaService {
    brokers: String,
    topics: Vec<String>,
    mongodb_service: MongodbService,
}
impl KafkaService {
    pub fn init(settings: KafkaSettings, mongodb_service: MongodbService) -> Self {
        KafkaService {
            brokers: settings.broker,
            topics: settings.consumer_topics,
            mongodb_service,
        }
    }

    // A type alias with your custom consumer can be created for convenience.

    pub(crate) async fn start_polling(&self) {
        let context = CustomContext;

        let consumer: LoggingConsumer = ClientConfig::new()
            .set("group.id", "noteapp_note_query_service")
            .set("bootstrap.servers", &self.brokers)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            //.set("statistics.interval.ms", "30000")
            //.set("auto.offset.reset", "smallest")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create_with_context(context)
            .expect("Consumer creation failed");
        let topics: Vec<&str> = self.topics.iter().map(|t| t as &str).collect();
        consumer
            .subscribe(&topics)
            .expect("Can't subscribe to specified topics");

        loop {
            match consumer.recv().await {
                Err(e) => warn!("Kafka error: {}", e),
                Ok(m) => {
                    
                   let payload =  m.payload().unwrap();
                   let event_model = serde_json::from_slice::<EventModel>(payload).unwrap();
                   self.apply_event(event_model).await;
                   consumer.commit_message(&m, CommitMode::Async).unwrap();
                }
            };
        }
    }

    pub(crate) async fn apply_event(&self, event: EventModel) {
        match event {
            EventModel::NoteCreatedEventModel(note_created_event_model) => {
                self.mongodb_service
                    .add_note(note_created_event_model)
                    .await
            }
            EventModel::ParentOfNoteChangedEventModel(parent_of_note_changed_event_model) => {
                self.mongodb_service
                    .change_parent_of_note(parent_of_note_changed_event_model)
                    .await
            }
            EventModel::BasicInfoOfNoteChangedEventModel(
                basic_info_of_note_changed_event_model,
            ) => {
                self.mongodb_service
                    .change_basic_info_of_note(basic_info_of_note_changed_event_model)
                    .await
            }
        }
    }
}

/* 
impl FromBytes for EventModel {
    type Error = Error;

    fn from_bytes(message: &[u8]) -> Result<&Self, Self::Error> {
        match  std::str::from_utf8(message) {
            Ok(payload) => { match serde_json::from_slice::<EventModel>(payload) {
                Ok(event_model) => { Ok(event_model)}
                Err(err) => {Err(erro)}
            } 
        
        }
            Err(err) => {}
        }
    }
}*/