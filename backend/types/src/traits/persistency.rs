use std::fmt::Debug;

pub trait IPeristency<T>: Debug + Send {
    fn load(&self) -> Vec<T>;
    fn save(&self, data: Vec<T>);
}
