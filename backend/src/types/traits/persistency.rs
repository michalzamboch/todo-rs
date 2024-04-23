use std::{error::Error, fmt::Debug};

pub trait IPeristency<T>: Debug + Send {
    fn load(&self) -> Result<Vec<T>, Box<dyn Error>>;
    fn save(&self, data: &[T]) -> Result<(), Box<dyn Error>>;
}
