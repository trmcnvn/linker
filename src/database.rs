use crate::models::Link;
use deadpool::managed::PoolConfig;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::{Config, NoTls};

#[derive(Clone)]
pub struct Database {
    pool: Pool,
}

impl Database {
    pub fn new(connection_url: &str) -> anyhow::Result<Self> {
        let pg_cfg = connection_url.parse::<Config>()?;
        let manager_cfg = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };
        let manager = Manager::from_config(pg_cfg, NoTls, manager_cfg);
        Ok(Self {
            pool: Pool::from_config(manager, PoolConfig::default()),
        })
    }

    pub async fn find(&self, id: &str) -> anyhow::Result<Link> {
        let client = self.pool.get().await?;
        let query = client
            .prepare("SELECT * FROM links WHERE short_id = $1")
            .await?;
        let row = client.query_one(&query, &[&id]).await?;
        Ok(Link::from(row))
    }

    pub async fn insert(&self, link: &Link) -> anyhow::Result<u64> {
        let client = self.pool.get().await?;
        let query = client
            .prepare("INSERT INTO links (external_url, short_id, created_at) VALUES ($1, $2, $3)")
            .await?;
        client
            .execute(
                &query,
                &[&link.external_url, &link.short_id, &link.created_at],
            )
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn find_or_insert(&self, link: &Link) -> anyhow::Result<Link> {
        match self.find(&link.short_id).await {
            Ok(link) => Ok(link),
            Err(_) => match self.insert(link).await {
                Ok(_) => Ok(link.clone()),
                Err(err) => {
                    println!("{:?}", err);
                    anyhow::bail!("Couldn't find or insert the link")
                }
            },
        }
    }
}
