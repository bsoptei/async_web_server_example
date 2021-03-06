use actix_web::*;
use async_trait::async_trait;
use futures::lock::Mutex;
use serde_json::Value;

use std::collections::HashMap;

pub mod handlers;
mod impls;

type Key = usize;
type Item = HashMap<String, Value>;

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
