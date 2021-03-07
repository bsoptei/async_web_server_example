use actix_web::*;
use async_trait::async_trait;
use futures::lock::Mutex;
use serde_json::Value;

use std::collections::HashMap;

pub mod handlers;
mod impls;
mod tests;

use handlers::*;

pub type Key = usize;
pub type Item = HashMap<String, Value>;

pub trait StoreProviderFeatures {
    type Store;
    fn store() -> Self::Store;
}

pub struct StoreProvider;

#[async_trait]
pub trait DataStore {
    async fn add(&self, item: Item) -> Option<Key>;
    async fn delete(&self, key: Key) -> Option<String>;
    async fn get(&self, key: Key) -> String;
    async fn get_all(&self) -> String;
    async fn replace(&self, key: Key, item: Item) -> Option<Key>;
    async fn update(&self, key: Key, item: Item) -> Option<Key>;
}

pub fn configure_endpoints<Store: 'static + DataStore>(cfg: &mut web::ServiceConfig) {
    cfg.route("/data", web::post().to(create::<Store>))
        .route("/data/{key}", web::delete().to(delete::<Store>))
        .route("/data/{key}", web::get().to(read::<Store>))
        .route("/data", web::get().to(read_all::<Store>))
        .route("/data/{key}", web::put().to(replace::<Store>))
        .route("/data/{key}", web::patch().to(update::<Store>));
}
