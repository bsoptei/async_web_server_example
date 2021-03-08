use actix_web::*;
use awse::*;

const PORT: &str = "127.0.0.1:8000";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let store = web::Data::new(StoreProvider::store());
    type Store = <StoreProvider as StoreProviderFeatures>::Store;

    HttpServer::new(move || {
        App::new()
            .app_data(store.clone())
            .configure(configure_endpoints::<Store>)
    })
    .bind(PORT)?
    .run()
    .await
}
