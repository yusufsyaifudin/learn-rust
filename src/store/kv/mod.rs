use std::fmt::Error;

// List of exported module so it can be accessed from the outer package.
pub mod entity;
pub mod inmem;

// Storage is a trait
pub trait Storage {
    fn put(&mut self, key: String, value: String) -> Result<bool, Error>;
    fn get(&mut self, key: String) -> Result<entity::KV, Error>;
}
