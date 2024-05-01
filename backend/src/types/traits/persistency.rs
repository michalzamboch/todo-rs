use async_trait::async_trait;
use std::{error::Error, fmt::Debug};

use serde::{de::DeserializeOwned, *};

pub trait IPeristency<T>: Debug + Send + Sync {
    fn load(&self) -> Result<Vec<T>, Box<dyn Error>>
    where
        T: DeserializeOwned;
    fn save(&self, data: &[T]) -> Result<(), Box<dyn Error>>
    where
        T: Sized + Serialize;
}

#[async_trait]
pub trait IPeristencyAsync<T>: Debug + Send + Sync {
    async fn load(&self) -> Result<Vec<T>, Box<dyn Error>>
    where
        T: DeserializeOwned;
    async fn save(&self, data: &[T]) -> Result<(), Box<dyn Error>>
    where
        T: Sized + Serialize;
}
