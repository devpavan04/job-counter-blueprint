use crate::{JobCounterError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobStats {
    pub total_jobs: u64,
    pub job_counts_by_id: HashMap<u64, u64>,
    pub last_processed: Option<String>,
}

impl Default for JobStats {
    fn default() -> Self {
        Self {
            total_jobs: 0,
            job_counts_by_id: HashMap::new(),
            last_processed: None,
        }
    }
}

pub struct JobStorage {
    stats: RwLock<JobStats>,
    storage_path: PathBuf,
}

impl JobStorage {
    pub fn new(storage_path: PathBuf) -> Result<Self> {
        let stats = if storage_path.exists() {
            let data = fs::read_to_string(&storage_path)?;
            serde_json::from_str(&data)?
        } else {
            JobStats::default()
        };

        Ok(Self {
            stats: RwLock::new(stats),
            storage_path,
        })
    }

    pub async fn increment_job_count(&self, job_id: u64, job_description: String) -> Result<u64> {
        let mut stats = self.stats.write().await;
        
        stats.total_jobs += 1;
        *stats.job_counts_by_id.entry(job_id).or_insert(0) += 1;
        stats.last_processed = Some(job_description);
        
        let total = stats.total_jobs;
        
        // Persist to disk
        self.save_stats(&stats).await?;
        
        tracing::info!(
            "Job processed! Total: {}, Job ID {}: {} times, Description: {}",
            total,
            job_id,
            stats.job_counts_by_id[&job_id],
            stats.last_processed.as_ref().unwrap()
        );
        
        Ok(total)
    }

    pub async fn get_stats(&self) -> JobStats {
        self.stats.read().await.clone()
    }

    pub async fn reset_counter(&self) -> Result<()> {
        let mut stats = self.stats.write().await;
        *stats = JobStats::default();
        self.save_stats(&stats).await?;
        
        tracing::info!("Job counter reset to 0");
        Ok(())
    }

    async fn save_stats(&self, stats: &JobStats) -> Result<()> {
        let data = serde_json::to_string_pretty(stats)?;
        fs::write(&self.storage_path, data)?;
        Ok(())
    }
}
