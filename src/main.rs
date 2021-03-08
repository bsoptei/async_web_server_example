use actix_web::*;
use awse::*;
use std::env;

const HOST: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "8000";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let port = match args.get(1) {
        Some(port_arg) => &port_arg,
        _ => DEFAULT_PORT
    };
    let host_port = HOST.to_owned() + ":" + port;
    let store = web::Data::new(StoreProvider::store());
    type Store = <StoreProvider as StoreProviderFeatures>::Store;

    HttpServer::new(move || {
        App::new()
            .app_data(store.clone())
            .configure(configure_endpoints::<Store>)
    })
    .bind(&host_port)?
    .run()
    .await
}
