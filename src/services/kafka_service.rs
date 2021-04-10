extern crate env_logger;
extern crate kafka;

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;

use crate::settings::KafkaSettings;

pub(crate) struct KafkaService {
    pub broker: String,
    pub topic: String,
}
impl KafkaService {
    pub fn init(settings: KafkaSettings) -> Self {
        KafkaService {
            broker: settings.broker,
            topic: settings.consumer_topics[0].to_string(),
        }
    }

    pub(crate) async fn consume_messages(
       &self
    ) -> Result<(), KafkaError> {
        let mut con = Consumer::from_hosts(vec![self.broker.to_owned()])
            
            .with_topic(self.topic.to_owned())
            .with_fallback_offset(FetchOffset::Earliest)
            .with_offset_storage(GroupOffsetStorage::Kafka)
            .create()?;

        loop {
            let mss = con.poll()?;
            if mss.is_empty() {
                println!("No messages available right now.");
            }

            for ms in mss.iter() {
                for m in ms.messages() {
                    println!(
                        "{}:{}@{}: {:?}",
                        ms.topic(),
                        ms.partition(),
                        m.offset,
                        m.value,
                        
                    );
                }
                let _ = con.consume_messageset(ms);
            }
            con.commit_consumed()?;
        }
    }
}
