use super::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

impl StoreProviderFeatures for StoreProvider {
    type Store = InMemoryStore;

    fn store() -> Self::Store {
        InMemoryStore::new()
    }
}

#[derive(Serialize, Deserialize)]
struct ItemWithKey {
    _key: Key,
    #[serde(flatten)]
    item: Item,
}

impl ItemWithKey {
    fn new(_key: Key, item: Item) -> Self {
        Self { _key, item }
    }
}

#[derive(Default)]
pub struct InMemoryStore {
    store: Mutex<HashMap<Key, String>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self::default()
    }

    async fn generate_key(&self) -> Key {
        let store = self.store.lock().await;
        let mut rng = rand::thread_rng();
        let mut key: Key = rng.gen();
        while store.contains_key(&key) {
            key = rng.gen();
        }
        key
    }
}

#[async_trait]
impl DataStore for InMemoryStore {
    async fn add(&self, item: Item) -> Option<Key> {
        let new_key = self.generate_key().await;
        match serde_json::to_string(&ItemWithKey::new(new_key, item)) {
            Ok(item_string) => {
                let mut store = self.store.lock().await;
                store.insert(new_key, item_string);
                Some(new_key)
            }
            _ => None,
        }
    }

    async fn delete(&self, key: Key) -> Option<String> {
        let mut store = self.store.lock().await;
        store.remove(&key)
    }

    async fn get(&self, key: Key) -> String {
        let store = &*self.store.lock().await;
        store
            .get(&key)
            .map(|elem| elem.to_owned())
            .unwrap_or_default()
    }

    async fn get_all(&self) -> String {
        let store = &*self.store.lock().await;
        serde_json::to_string(&store.values().collect::<Vec<&String>>()).unwrap_or_default()
    }

    async fn replace(&self, key: Key, item: Item) -> Option<Key> {
        let mut store = self.store.lock().await;
        match store.get_mut(&key) {
            Some(elem) => match serde_json::from_str::<ItemWithKey>(elem) {
                Ok(mut deserialized) => {
                    deserialized.item = item;
                    *elem = serde_json::to_string(&deserialized).unwrap();
                    Some(key)
                }
                _ => None,
            },
            _ => None,
        }
    }

    async fn update(&self, key: Key, item: Item) -> Option<Key> {
        let mut store = self.store.lock().await;
        match store.get_mut(&key) {
            Some(elem) => match serde_json::from_str::<ItemWithKey>(elem) {
                Ok(mut deserialized) => {
                    for (key, new_value) in item.into_iter() {
                        match deserialized.item.get_mut(&key) {
                            Some(value) => {
                                *value = new_value;
                            }
                            _ => {
                                // this branch currently allows the addition of new keys to the item upon update
                                deserialized.item.insert(key, new_value);
                            }
                        }
                    }
                    *elem = serde_json::to_string(&deserialized).unwrap();
                    Some(key)
                }
                _ => None,
            },
            _ => None,
        }
    }
}
