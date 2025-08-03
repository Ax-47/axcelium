use crate::{
    application::repositories::{
        cdc::utils::{
            get_bool, get_i64, get_optional_i64, get_optional_string, get_string, get_uuid,
        },
        errors::replicator::ReplicatorRepoError,
    },
    domain::entities::user::User,
    infrastructure::{
        errors::queue::ReplicatorError, models::queue::users::QueueUser,
        repositories::queue::producer::ProducerRepository,
    },
};
use anyhow::Result;
use async_trait::async_trait;
use scylla_cdc::consumer::CDCRow;
use std::sync::{Arc, Mutex};

pub struct ReplicatorRepositoryImpl {
    producer: Arc<Mutex<dyn ProducerRepository>>,
}
impl ReplicatorRepositoryImpl {
    pub fn new(producer: Arc<Mutex<dyn ProducerRepository>>, topic: String) -> Self {
        let instance = Self {
            producer: producer.clone(),
        };

        if let Ok(mut locked) = producer.lock() {
            locked.set_topic(&topic);
        } else {
            // ถ้า lock ไม่ได้ จะ panic หรือ log ก็แล้วแต่ว่าจะจัดการยังไง
            panic!("Failed to acquire lock on producer to set topic");
        }

        instance
    }
    fn send(&self, data: QueueUser) -> Result<(), ReplicatorError> {
        let json = serde_json::to_string(&data)?;
        println!("{json}");
        let mut locked = self
            .producer
            .lock()
            .map_err(|_| ReplicatorError::LockError)?;
        locked
            .send_data_to_topic(json)
            .map_err(|_| ReplicatorError::KafkaSendError)?;
        Ok(())
    }
}

#[async_trait]
pub trait ReplicatorRepository: Send + Sync {
    fn create(&self, user: User) -> Result<(), ReplicatorError>;
    fn update(&self, user: User) -> Result<(), ReplicatorError>;
    fn delete(&self) -> Result<(), ReplicatorError>; // หรือจะรับ user_id ก็ได้
    //
    fn parse_user_from_cdcrow(&self, data: &CDCRow<'_>) -> Result<User, ReplicatorRepoError>;
}

#[async_trait]
impl ReplicatorRepository for ReplicatorRepositoryImpl {
    fn create(&self, user: User) -> Result<(), ReplicatorError> {
        let message = QueueUser::new("create", user);
        self.send(message)
    }

    fn update(&self, user: User) -> Result<(), ReplicatorError> {
        let message = QueueUser::new("update", user);
        self.send(message)
    }

    fn delete(&self) -> Result<(), ReplicatorError> {
        let message = QueueUser::delete("delete");
        self.send(message)
    }
    fn parse_user_from_cdcrow(&self, data: &CDCRow<'_>) -> Result<User, ReplicatorRepoError> {
        Ok(User {
            user_id: get_uuid(data, "user_id")?,
            organization_id: get_uuid(data, "organization_id")?,
            application_id: get_uuid(data, "application_id")?,
            username: get_string(data, "username")?,
            email: get_optional_string(data, "email")?,
            hashed_password: get_string(data, "hashed_password")?,
            created_at: get_i64(data, "created_at")?,
            updated_at: get_i64(data, "updated_at")?,
            is_active: get_bool(data, "is_active")?,
            is_verified: get_bool(data, "is_verified")?,
            is_locked: get_bool(data, "is_locked")?,
            last_login: get_optional_i64(data, "last_login")?,
            mfa_enabled: get_bool(data, "mfa_enabled")?,
            deactivated_at: get_optional_i64(data, "deactivated_at")?,
            locked_at: get_optional_i64(data, "locked_at")?,
        })
    }
}
