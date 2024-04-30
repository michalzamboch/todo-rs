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
