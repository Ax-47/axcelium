use crate::infrastructure::repositories::queue::consumer::ConsumerRepository;
pub struct UserConsumerRepositoryImpl {
    consumer_repo: Box<dyn ConsumerRepository>,
}

impl UserConsumerRepositoryImpl {
    pub fn new(consumer_repo: Box<dyn ConsumerRepository>) -> Self {
        Self { consumer_repo }
    }

    pub fn consume_until(&mut self) {
        while self.consumer_repo.is_running() {
            match self.consumer_repo.consume_events() {
                Ok(messagesets) => {
                    for ms in messagesets.iter() {
                        for m in ms.messages() {
                            match std::str::from_utf8(m.value) {
                                Ok(v) => println!("{:?}", v),
                                Err(e) => eprintln!("UTF-8 error: {:?}", e),
                            }
                        }

                        if let Err(e) = self.consumer_repo.consume_messageset(ms) {
                            eprintln!("consume_messageset error: {:?}", e);
                        }
                    }

                    if let Err(e) = self.consumer_repo.commit_consumed() {
                        eprintln!("commit_consumed error: {:?}", e);
                    }
                }
                Err(e) => {
                    eprintln!("consume_events error: {:?}", e);
                }
            }
        }

        println!("Kafka consumer stopped~!");
    }
}
