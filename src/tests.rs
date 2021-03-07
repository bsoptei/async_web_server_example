#[cfg(test)]
mod tests {
    use super::super::*;
    use actix_web::{test, web, App};

    struct MockStore {}

    impl MockStore {
        const MOCK_KEY: Key = 777;
        const MOCK_ITEM_STR: &'static str = "";
    }

    #[async_trait]
    impl DataStore for MockStore {
        async fn add(&self, _item: Item) -> Option<Key> {
            Some(MockStore::MOCK_KEY)
        }

        async fn delete(&self, _key: Key) -> Option<String> {
            Some(MockStore::MOCK_ITEM_STR.to_owned())
        }

        async fn get(&self, _key: Key) -> String {
            MockStore::MOCK_ITEM_STR.to_owned()
        }

        async fn get_all(&self) -> String {
            MockStore::MOCK_ITEM_STR.to_owned()
        }

        async fn replace(&self, _key: Key, _item: Item) -> Option<Key> {
            Some(MockStore::MOCK_KEY)
        }

        async fn update(&self, _key: Key, _item: Item) -> Option<Key> {
            Some(MockStore::MOCK_KEY)
        }
    }

    #[actix_rt::test]
    async fn test_app_endpoints() {
        let mock_store = web::Data::new(MockStore {});
        let mut app = test::init_service(
            App::new()
                .app_data(mock_store.clone())
                .configure(configure_endpoints::<MockStore>),
        )
        .await;
        let data = Item::new();

        let req = test::TestRequest::post().uri("/data").set_json(&data).to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}
