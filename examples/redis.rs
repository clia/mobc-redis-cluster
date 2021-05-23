use mobc::Pool;
use mobc_redis_cluster::RedisClusterConnectionManager;
use mobc_redis_cluster::{Connection};
use std::time::Instant;
use redis_cluster_async::{Client, redis::cmd};


#[tokio::main]
async fn main() {
    let nodes = vec!["redis://127.0.0.1:7000", "redis://127.0.0.1:7001", "redis://127.0.0.1:7002", "redis://127.0.0.1:7003", "redis://127.0.0.1:7004", "redis://127.0.0.1:7005"];

    let client = Client::open(nodes).unwrap();
    let manager = RedisClusterConnectionManager::new(client);
    let pool = Pool::builder().max_open(100).build(manager);

    const MAX: usize = 5000;

    let now = Instant::now();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<usize>(16);
    for i in 0..MAX {
        let pool = pool.clone();
        let mut tx_c = tx.clone();
        tokio::spawn(async move {
            let mut conn = pool.get().await.unwrap();
            let s: String = redis::cmd("PING")
                .query_async(&mut conn as &mut Connection)
                .await
                .unwrap();
            assert_eq!(s.as_str(), "PONG");
            tx_c.send(i).await.unwrap();
        });
    }
    for _ in 0..MAX {
        rx.recv().await.unwrap();
    }

    println!("cost: {:?}", now.elapsed());
}
