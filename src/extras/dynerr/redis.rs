#[cfg(feature="dynerr")]
use r2d2::Pool;

#[cfg(feature="dynerr")]
use redis::cluster::ClusterClient;

#[cfg(feature="dynerr")]
use redis::{Client, FromRedisValue, RedisError, Commands};

#[cfg(feature="dynerr")]
use crate::{redis_settings, Interpolate, StandardError, Result};

#[cfg(feature="dynerr")]
impl From<RedisError> for StandardError{
    fn from(err: RedisError) -> Self {
        StandardError::new("ER-REDIS").interpolate_err(format!("{:?}", &err))
    }
}

#[cfg(all(feature = "dynerr", not(feature = "diesel")))]
impl From<r2d2::Error> for StandardError{
    fn from(err: r2d2::Error) -> Self {
        StandardError::new("ER-REDIS").interpolate_err(format!("{:?}", &err))
    }
}


#[derive(Clone)]
pub enum RedisBackend {
    Redis(Pool<Client>),
    RedisCluster(Pool<ClusterClient>),
}

impl RedisBackend {
    pub fn new() -> Result<RedisBackend> {
        if redis_settings.use_redis_cluster {
            let nodes = vec![redis_settings.cache_location.clone()];
            let cluster_client = ClusterClient::new(nodes)?;
            let cluster_pool = Pool::builder()
                .max_size(5)
                .connection_timeout(redis_settings.cache_timeout)
                .build(cluster_client)?;
            Ok(RedisBackend::RedisCluster(cluster_pool))
        } else {
            let client = Client::open(redis_settings.cache_location.clone())?;
            let pool = Pool::builder()
                .max_size(5)
                .connection_timeout(redis_settings.cache_timeout)
                .build(client)?;
            Ok(RedisBackend::Redis(pool))
        }
    }

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
