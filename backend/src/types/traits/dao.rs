use std::{error::Error, fmt::Debug, sync::*};

use super::persistency::ILoadable;

pub trait IDao<T>: Debug + Send {
    fn select_by(&self, id: u32) -> Option<T>;
    fn get_all(&self) -> Vec<T>;

    fn insert_row(&self, item: &T) -> Result<(), Box<dyn Error>>;
    fn update_row(&self, item: &T) -> Result<(), Box<dyn Error>>;
    fn remove_row(&self, id: u32) -> Result<(), Box<dyn Error>>;

    fn count(&self) -> u32;
    fn max_id(&self) -> u32;
    fn exists(&self, id: u32) -> bool;
}

pub type DaoRef<T> = Arc<Box<dyn IDao<T>>>;

pub trait IDaoThreadSafe<T>: Debug + Send + Sync + ILoadable<T> {
    fn select_by(&self, id: u32) -> Option<T>;
    fn get_all(&self) -> Vec<T>;

    fn insert_row(&mut self, item: &T) -> Result<(), Box<dyn Error>>;
    fn update_row(&mut self, item: &T) -> Result<(), Box<dyn Error>>;
    fn remove_row(&mut self, id: u32) -> Result<(), Box<dyn Error>>;

    fn count(&self) -> u32;
    fn max_id(&self) -> u32;
    fn exists(&self, id: u32) -> bool;
}

pub type DaoThreadSafeRef<T> = Arc<RwLock<dyn IDaoThreadSafe<T>>>;
