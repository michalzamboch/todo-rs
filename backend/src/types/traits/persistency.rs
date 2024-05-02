use async_trait::async_trait;
use std::{error::Error, fmt::Debug, ops::Deref};

use serde::{de::DeserializeOwned, *};

pub trait IPeristency<T>: Debug + Send + Sync {
    fn load(&self) -> Result<Vec<T>, Box<dyn Error>>
    where
        T: DeserializeOwned;
    fn save(&self, data: &[T]) -> Result<(), Box<dyn Error>>
    where
        T: Sized + Serialize;
}


pub struct BoxedSendError(Box<dyn Error + Send>);
impl Deref for BoxedSendError {
    type Target = Box<dyn Error + Send>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<E: Error + Send + 'static> From<E> for BoxedSendError {
    fn from(e: E) -> Self {
        BoxedSendError(Box::new(e))
    }
}

#[async_trait]
pub trait IPeristencyAsync<T>: Debug + Send + Sync {
    async fn load(&self) -> Result<Vec<T>, BoxedSendError>
    where
        T: DeserializeOwned;
    async fn save(&self, data: &[T]) -> Result<(), BoxedSendError>
    where
        T: Sized + Serialize;
}
