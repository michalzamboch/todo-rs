use std::{error::Error, fmt::Debug};

pub trait IDao<T>: Debug + Send {
    /*
    public T selectBy(int id);
	public Collection<T> getAll();
	public T insertRow(T obj);
	public boolean updateRow(T obj);
    */

    fn select_by(&self, id: u32) -> Option<T>;

    fn insert_row(&mut self, item: T) -> Option<T>;
    fn update_row(&mut self, item: T) -> Result<T, Box<dyn Error>>;


    fn count(&self) -> u32;
    fn max_id(&self) -> u32;
}
