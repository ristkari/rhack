pub mod connection_manager {
    use redis::aio::ConnectionManager;
    use redis::{Client, RedisError};
    use std::env;

    pub async fn get_connection_manager() -> Result<redis::aio::ConnectionManager, RedisError> {
        let redis_host_name =
            env::var("REDIS_HOSTNAME").expect("Missing env variable REDIS_HOSTNAME");
        let redis_tls = env::var("REDIS_TLS")
            .map(|redis_tls| redis_tls == "1")
            .unwrap_or(false);
        let uri_scheme = match redis_tls {
            true => "rediss",
            false => "redis",
        };

        let redis_conn_url = format!("{}://{}", uri_scheme, redis_host_name);
        let client = redis::Client::open(redis_conn_url).expect("Invalid redis URL");
        match client.get_tokio_connection_manager().await {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }
}
