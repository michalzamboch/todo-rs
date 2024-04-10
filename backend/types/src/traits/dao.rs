use std::{error::Error, fmt::Debug};

pub trait IDao<T>: Debug + Send {
    fn select_by(&self, id: u32) -> Option<T>;
    fn get_all(&self) -> Vec<T>;

    fn insert_row(&self, item: T) -> Result<T, Box<dyn Error>>;
    fn update_row(&self, item: T) -> Result<T, Box<dyn Error>>;
    fn remove_row(&self, id: u32) -> Result<(), Box<dyn Error>>;

    fn count(&self) -> u32;
    fn max_id(&self) -> u32;
}
