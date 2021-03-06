use awse::{handlers::*, *};
use actix_web::*;

const PORT: &str = "127.0.0.1:8000";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let store = web::Data::new(StoreProvider::store());
    type Store = <StoreProvider as StoreProviderFeatures>::Store;

    HttpServer::new(move || {
        App::new()
            .app_data(store.clone())
            .route("/data", web::post().to(create::<Store>))
            .route("/data/{key}", web::delete().to(delete::<Store>))
            .route("/data/{key}", web::get().to(read::<Store>))
            .route("/data", web::get().to(read_all::<Store>))
            .route("/data/{key}", web::put().to(replace::<Store>))
            .route("/data/{key}", web::patch().to(update::<Store>))
    })
    .bind(PORT)?
    .run()
    .await
}
