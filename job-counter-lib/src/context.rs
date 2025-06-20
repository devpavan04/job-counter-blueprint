use crate::{storage::JobStorage, Result};
use blueprint_sdk::prelude::*;
use std::sync::Arc;

#[derive(Clone, TangleClientContext, ServicesContext)]
pub struct JobCounterContext {
    #[config]
    pub env: BlueprintEnvironment,
    pub storage: Arc<JobStorage>,
}

impl JobCounterContext {
    pub async fn new(env: BlueprintEnvironment) -> Result<Self> {
        let data_dir = env.data_dir()?;
        let storage_path = data_dir.join("job_stats.json");
        
        // Ensure data directory exists
        std::fs::create_dir_all(&data_dir)?;
        
        let storage = Arc::new(JobStorage::new(storage_path)?);
        
        Ok(Self { env, storage })
    }
}
