use actix_web::{web, App, HttpServer, Responder};
use mobc_redis_cluster::RedisClusterConnectionManager;
use mobc_redis_cluster::{redis, Connection};

use redis_cluster_async::{Client, redis::cmd};

type Pool = mobc::Pool<RedisClusterConnectionManager>;

async fn ping(pool: web::Data<Pool>) -> impl Responder {
        let mut conn = pool.get().await.unwrap();
    match redis::cmd("PING")
        .query_async::<_, String>(&mut conn as &mut Connection)
        .await
    {
        Ok(pong) => pong,
        Err(e) => format!("Server error: {:?}", e),
    }
}

#[actix_rt::main]
async fn main() {
    let nodes = vec!["redis://127.0.0.1:7000", "redis://127.0.0.1:7001", "redis://127.0.0.1:7002", "redis://127.0.0.1:7003", "redis://127.0.0.1:7004", "redis://127.0.0.1:7005"];

    let client = Client::open(nodes).unwrap();
    let manager = RedisClusterConnectionManager::new(client);
    let pool = Pool::builder().max_open(100).build(manager);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(ping))
    })
    .bind("127.0.0.1:7777")
    .unwrap()
    .run()
    .await
    .unwrap();
}
