use crate::{
    domain::entities::user::User,
    infrastructure::{
        errors::queue::ProducerError,
        models::queue::users::QueueUser,
        repositories::queue::{producer::ProducerRepository, topics::USER_TOPIC},
    },
};

pub struct UserProducerRepositoryImpl {
    producer_repo: Box<dyn ProducerRepository>,
}

impl UserProducerRepositoryImpl {
    pub fn new(mut producer_repo: Box<dyn ProducerRepository>) -> Self {
        producer_repo.set_topic(USER_TOPIC);
        Self { producer_repo }
    }
}
pub trait UserProducerRepository: Send + Sync {
    fn create(&mut self, user: User) -> Result<(), ProducerError>;
}
impl UserProducerRepository for UserProducerRepositoryImpl {
    fn create(&mut self, user: User) -> Result<(), ProducerError> {
        let data = QueueUser {
            operation: "create".to_string(),
            user: Some(user),
        };
        let json_string = serde_json::to_string(&data).unwrap();
        self.producer_repo.send_data_to_topic(json_string)?;
        Ok(())
    }
}
