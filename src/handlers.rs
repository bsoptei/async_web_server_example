use super::*;

pub async fn create<DS: DataStore>(json: web::Json<Item>, store: web::Data<DS>) -> HttpResponse {
    match store.add(json.into_inner()).await {
        Some(key) => HttpResponse::Created().body(serde_json::to_string(&key).unwrap_or_default()),
        _ => HttpResponse::BadRequest().finish(),
    }
}

pub async fn delete<DS: DataStore>(path: web::Path<Key>, store: web::Data<DS>) -> HttpResponse {
    let delete_result = store.delete(path.into_inner()).await;
    HttpResponse::Ok().body(serde_json::to_string(&delete_result).unwrap_or_default())
}

pub async fn read<DS: DataStore>(path: web::Path<Key>, store: web::Data<DS>) -> HttpResponse {
    HttpResponse::Ok().body(store.get(path.into_inner()).await)
}

pub async fn read_all<DS: DataStore>(store: web::Data<DS>) -> HttpResponse {
    HttpResponse::Ok().body(store.get_all().await)
}

pub async fn replace<DS: DataStore>(
    json: web::Json<Item>,
    path: web::Path<Key>,
    store: web::Data<DS>,
) -> HttpResponse {
    let replace_result = store.replace(path.into_inner(), json.into_inner()).await;
    HttpResponse::Ok().body(serde_json::to_string(&replace_result).unwrap_or_default())
}

pub async fn update<DS: DataStore>(
    json: web::Json<Item>,
    path: web::Path<Key>,
    store: web::Data<DS>,
) -> HttpResponse {
    let update_result = store.update(path.into_inner(), json.into_inner()).await;
    HttpResponse::Ok().body(serde_json::to_string(&update_result).unwrap_or_default())
}
