use actix_web::*;
use awse::*;
use std::env;

const HOST: &str = "0.0.0.0";
const DEFAULT_PORT: &str = "8080";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let host = env::var("HOST").ok().unwrap_or(HOST.to_owned());
    let port = env::var("PORT").ok().unwrap_or(DEFAULT_PORT.to_owned());  
    let store = web::Data::new(StoreProvider::store());
    type Store = <StoreProvider as StoreProviderFeatures>::Store;

    HttpServer::new(move || {
        App::new()
            .app_data(store.clone())
            .configure(configure_endpoints::<Store>)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
