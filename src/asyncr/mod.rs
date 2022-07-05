use redis::{aio::Connection, FromRedisValue, RedisError};
use redis::{
    from_redis_value,
    streams::{StreamRangeReply, StreamReadOptions, StreamReadReply},
    AsyncCommands, Client,
};
use std::env;
use std::{error::Error, time::Duration};

use redis::aio::ConnectionManager;

pub async fn get_connection_manager() -> redis::aio::ConnectionManager {
    let redis_host_name =
        env::var("REDIS_HOSTNAME").expect("Missing environment variable REDIS_HOSTNAME");
    let redis_tls = env::var("REDIS_TLS")
        .map(|redis_tls| redis_tls == "1")
        .unwrap_or(false);
    let uri_scheme = match redis_tls {
        true => "rediss",
        false => "redis",
    };

    let redis_conn_url = format!("{}://{}", uri_scheme, redis_host_name);
    let client = redis::Client::open(redis_conn_url).expect("Invalid connection URL");
    let redis = client.get_tokio_connection_manager().await?.unwrap();
    //.unwrap();
    //let redis = ConnectionManager::new(client.clone()).await.unwrap();
    redis
}
