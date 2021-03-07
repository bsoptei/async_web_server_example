use super::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

impl StoreProviderFeatures for StoreProvider {
    type Store = InMemoryStore;

    fn store() -> Self::Store {
        InMemoryStore::new()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    use maplit::*;
    use serde_json::Number;

    #[test]
    fn test_in_memory_store() {
        let test_store = InMemoryStore::new();
        let get_all_first = block_on(test_store.get_all());
        assert_eq!("[]", &get_all_first);

        let field = || String::from("age");
        let value = || Value::Number(Number::from(42));
        let item = || hashmap![field() => value()];

        let key_opt = block_on(test_store.add(item()));
        assert!(key_opt.is_some());
        let key = key_opt.unwrap();

        let item_retrieved: ItemWithKey =
            serde_json::from_str(&block_on(test_store.get(key))).unwrap();

        let expected_item_with_key = ItemWithKey::new(key, item());

        assert_eq!(expected_item_with_key, item_retrieved);

        let get_all_second: Vec<String> =
            serde_json::from_str(&block_on(test_store.get_all())).unwrap();
        let item_retrieved_differently: ItemWithKey =
            serde_json::from_str(get_all_second.get(0).unwrap()).unwrap();

        assert_eq!(expected_item_with_key, item_retrieved_differently);

        let field2 = || String::from("id");
        let value2 = || Value::Number(Number::from(123456));
        let value3 = || Value::Number(Number::from(66));
        let item2 = || hashmap![field() => value3(), field2() => value2()];

        let non_existing_key = 123;

        let key_opt2 = block_on(test_store.update(key, item2()));
        assert!(key_opt2.is_some());
        let key2 = key_opt2.unwrap();

        let key_opt_none = block_on(test_store.update(non_existing_key, item2()));
        assert!(key_opt_none.is_none());

        assert_eq!(key, key2);

        let expected_item_with_key2 =
            ItemWithKey::new(key2, hashmap![field() => value3(), field2() => value2()]);

        let item_retrieved2: ItemWithKey =
            serde_json::from_str(&block_on(test_store.get(key))).unwrap();

        assert_eq!(expected_item_with_key2, item_retrieved2);

        let key_opt3 = block_on(test_store.replace(key, item()));
        assert!(key_opt3.is_some());
        let key3 = key_opt3.unwrap();
        assert_eq!(key2, key3);

        let item_retrieved3: ItemWithKey =
            serde_json::from_str(&block_on(test_store.get(key))).unwrap();

        assert_eq!(expected_item_with_key, item_retrieved3);

        let key_opt_none2 = block_on(test_store.replace(non_existing_key, item()));
        assert!(key_opt_none2.is_none());

        let delete_result1: ItemWithKey =
            serde_json::from_str(&block_on(test_store.delete(key)).unwrap()).unwrap();
        assert_eq!(expected_item_with_key, delete_result1);

        let delete_result2 = block_on(test_store.delete(key));
        assert!(delete_result2.is_none());

        let item_retrieved_last = block_on(test_store.get(key));
        assert_eq!("", &item_retrieved_last);

        let get_all_last = block_on(test_store.get_all());
        assert_eq!("[]", &get_all_last);
    }
}
