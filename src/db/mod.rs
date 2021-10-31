pub mod dynamodb;
pub mod mongodb;
pub mod postgres;
pub mod shared;
pub mod sqlite;

pub use shared::Adapter;
pub use shared::Client;
