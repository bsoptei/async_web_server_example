#[cfg(test)]
mod tests {
    use crate::*;
    use actix_web::{test, web, App};
    use futures_util::stream::TryStreamExt;

    struct MockStore {}

    impl MockStore {
        const MOCK_KEY_CREATE: &'static str = "777";
        const MOCK_KEY_NONEXISTING: &'static str = "0";
        const MOCK_KEY_UPDATE: &'static str = "42";
        const MOCK_KEY_REPLACE: &'static str = "666";
        const MOCK_KEY_DELETE: &'static str = "259";
        const MOCK_ITEM_STR_PRESENT: &'static str = "{}";
        const MOCK_ITEM_STR_NOT_PRESENT: &'static str = "";
        const MOCK_MULTI_ITEM_STR: &'static str = "[]";
    }

    fn str_as_key(key: &str) -> Key {
        Key::from_str_radix(key, 10).unwrap()
    }

    #[async_trait]
    impl DataStore for MockStore {
        async fn add(&self, _item: Item) -> Option<Key> {
            Some(str_as_key(MockStore::MOCK_KEY_CREATE))
        }

        async fn delete(&self, _key: Key) -> Option<Key> {
            Some(str_as_key(MockStore::MOCK_KEY_DELETE))
        }

        async fn get(&self, key: Key) -> String {
            if str_as_key(MockStore::MOCK_KEY_CREATE) == key {
                MockStore::MOCK_ITEM_STR_PRESENT.to_owned()
            } else {
                MockStore::MOCK_ITEM_STR_NOT_PRESENT.to_owned()
            }
        }

        async fn get_all(&self) -> String {
            MockStore::MOCK_MULTI_ITEM_STR.to_owned()
        }

        async fn replace(&self, _key: Key, _item: Item) -> Option<Key> {
            Some(str_as_key(MockStore::MOCK_KEY_REPLACE))
        }

        async fn update(&self, _key: Key, _item: Item) -> Option<Key> {
            Some(str_as_key(MockStore::MOCK_KEY_UPDATE))
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
        let post_req = test::TestRequest::post()
            .uri(DATA_ENDPOINT_PATTERN)
            .set_json(&data)
            .to_request();
        let mut resp_from_post = test::call_service(&mut app, post_req).await;
        assert!(resp_from_post.status().is_success());

        let body_from_post = test::load_stream(resp_from_post.take_body().into_stream()).await;
        assert_eq!(
            web::Bytes::from_static(MockStore::MOCK_KEY_CREATE.as_bytes()),
            body_from_post.unwrap()
        );

        let url_with_key = DATA_ENDPOINT_PATTERN.to_owned() + "/" + MockStore::MOCK_KEY_CREATE;

        let get_req1 = test::TestRequest::get().uri(&url_with_key).to_request();

        let mut resp_from_get1 = test::call_service(&mut app, get_req1).await;
        assert!(resp_from_get1.status().is_success());

        let body_from_get1 = test::load_stream(resp_from_get1.take_body().into_stream()).await;
        assert_eq!(
            web::Bytes::from_static(MockStore::MOCK_ITEM_STR_PRESENT.as_bytes()),
            body_from_get1.unwrap()
        );

        let url_with_nonexisting_key =
            DATA_ENDPOINT_PATTERN.to_owned() + "/" + MockStore::MOCK_KEY_NONEXISTING;

        let get_req2 = test::TestRequest::get()
            .uri(&url_with_nonexisting_key)
            .to_request();

        let mut resp_from_get2 = test::call_service(&mut app, get_req2).await;
        assert!(resp_from_get2.status().is_success());

        let body_from_get2 = test::load_stream(resp_from_get2.take_body().into_stream()).await;
        assert_eq!(
            web::Bytes::from_static(MockStore::MOCK_ITEM_STR_NOT_PRESENT.as_bytes()),
            body_from_get2.unwrap()
        );

        let get_req_all = test::TestRequest::get()
            .uri(&DATA_ENDPOINT_PATTERN)
            .to_request();

        let mut resp_from_get_all = test::call_service(&mut app, get_req_all).await;
        assert!(resp_from_get_all.status().is_success());

        let body_from_get_all =
            test::load_stream(resp_from_get_all.take_body().into_stream()).await;
        assert_eq!(
            web::Bytes::from_static(MockStore::MOCK_MULTI_ITEM_STR.as_bytes()),
            body_from_get_all.unwrap()
        );

        let update_req = test::TestRequest::patch()
            .uri(&url_with_key)
            .set_json(&data)
            .to_request();

        let mut resp_from_update = test::call_service(&mut app, update_req).await;
        assert!(resp_from_update.status().is_success());

        let body_from_update = test::load_stream(resp_from_update.take_body().into_stream()).await;
        assert_eq!(
            web::Bytes::from_static(MockStore::MOCK_KEY_UPDATE.as_bytes()),
            body_from_update.unwrap()
        );

        let replace_req = test::TestRequest::put()
            .uri(&url_with_key)
            .set_json(&data)
            .to_request();

        let mut resp_from_replace = test::call_service(&mut app, replace_req).await;
        assert!(resp_from_replace.status().is_success());

        let body_from_replace =
            test::load_stream(resp_from_replace.take_body().into_stream()).await;
        assert_eq!(
            web::Bytes::from_static(MockStore::MOCK_KEY_REPLACE.as_bytes()),
            body_from_replace.unwrap()
        );

        let delete_req = test::TestRequest::delete().uri(&url_with_key).to_request();

        let mut resp_from_delete = test::call_service(&mut app, delete_req).await;
        assert!(resp_from_delete.status().is_success());

        let body_from_delete = test::load_stream(resp_from_delete.take_body().into_stream()).await;
        assert_eq!(
            web::Bytes::from_static(MockStore::MOCK_KEY_DELETE.as_bytes()),
            body_from_delete.unwrap()
        );
    }
}
