use crate::{context::JobCounterContext, Result};
use blueprint_sdk::prelude::*;

pub const GET_STATS_JOB_ID: u64 = 1;

#[debug_job]
pub async fn get_stats(
    Context(ctx): Context<JobCounterContext>,
) -> Result<TangleResult<String>> {
    let job_description = "Getting job statistics".to_string();
    
    // Get current stats before incrementing (since this is also a job)
    let current_stats = ctx.storage.get_stats().await;
    
    // Increment counter for this stats request
    let total_count = ctx.storage.increment_job_count(GET_STATS_JOB_ID, job_description).await?;
    
    let stats_json = serde_json::to_string_pretty(&current_stats)?;
    let result = format!(
        "Job Statistics (before this request):\n{}\n\nTotal jobs now: {}",
        stats_json,
        total_count
    );
    
    Ok(TangleResult::ok(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::JobCounterContext;
    use blueprint_sdk::testing::TangleTestHarness;

    #[tokio::test]
    async fn test_get_stats() {
        let harness = TangleTestHarness::new().await.unwrap();
        let ctx = JobCounterContext::new(harness.env()).await.unwrap();
        
        let result = get_stats(Context(ctx.clone())).await.unwrap();
        
        assert!(result.is_ok());
        let stats = ctx.storage.get_stats().await;
        assert_eq!(stats.total_jobs, 1);
        assert_eq!(stats.job_counts_by_id[&GET_STATS_JOB_ID], 1);
    }
}
