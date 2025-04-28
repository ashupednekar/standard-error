use r2d2::Pool;
use redis::cluster::ClusterClient;
use redis::{Client, FromRedisValue, RedisError, Commands};
use std::time::Duration;

use crate::redis_settings;

#[derive(Debug)]
pub enum RedisErr {
    RedisConnectionError(RedisError),
    R2d2ConnectionError(r2d2::Error),
}

impl From<RedisError> for RedisErr {
    fn from(err: RedisError) -> Self {
        RedisErr::RedisConnectionError(err)
    }
}

impl From<r2d2::Error> for RedisErr {
    fn from(err: r2d2::Error) -> Self {
        RedisErr::R2d2ConnectionError(err)
    }
}

pub type Result<T> = core::result::Result<T, RedisErr>;

#[derive(Clone)]
pub enum RedisBackend {
    Redis(Pool<Client>),
    RedisCluster(Pool<ClusterClient>),
}

pub fn init_redis_connection_pool() -> Result<RedisBackend> {
    if redis_settings.use_redis_cluster {
        let nodes = vec![redis_settings.cache_location.clone()];
        let cluster_client = ClusterClient::new(nodes)?;
        let cluster_pool = Pool::builder()
            .max_size(5)
            .connection_timeout(Duration::from_secs(redis_settings.cache_timeout))
            .build(cluster_client)?;
        Ok(RedisBackend::RedisCluster(cluster_pool))
    } else {
        let client = Client::open(redis_settings.cache_location.clone())?;
        let pool = Pool::builder()
            .max_size(5)
            .connection_timeout(Duration::from_secs(redis_settings.cache_timeout))
            .build(client)?;
        Ok(RedisBackend::Redis(pool))
    }
}

impl RedisBackend {
    pub fn get<T: FromRedisValue>(&self, key: &str) -> Result<Option<T>> {
        match self {
            RedisBackend::Redis(pool) => {
                let mut conn = pool.get()?;
                let value = redis::cmd("GET").arg(key).query::<Option<T>>(&mut *conn)?;
                Ok(value)
            }
            RedisBackend::RedisCluster(pool) => {
                let mut conn = pool.get()?;
                let value = redis::cmd("GET").arg(key).query::<Option<T>>(&mut *conn)?;
                Ok(value)
            }
        }
    }
}
