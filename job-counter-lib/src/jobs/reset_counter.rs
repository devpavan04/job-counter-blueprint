use crate::{context::JobCounterContext, Result};
use blueprint_sdk::prelude::*;

pub const RESET_COUNTER_JOB_ID: u64 = 2;

#[debug_job]
pub async fn reset_counter(
    Context(ctx): Context<JobCounterContext>,
) -> Result<TangleResult<String>> {
    let job_description = "Resetting job counter".to_string();
    
    // Reset the counter
    ctx.storage.reset_counter().await?;
    
    // This is a bit special - we increment after reset to count this reset job
    let total_count = ctx.storage.increment_job_count(RESET_COUNTER_JOB_ID, job_description).await?;
    
    let result = format!("Job counter has been reset. Current count: {}", total_count);
    
    Ok(TangleResult::ok(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{context::JobCounterContext, jobs::process_data::{process_data, PROCESS_DATA_JOB_ID}};
    use blueprint_sdk::testing::TangleTestHarness;

    #[tokio::test]
    async fn test_reset_counter() {
        let harness = TangleTestHarness::new().await.unwrap();
        let ctx = JobCounterContext::new(harness.env()).await.unwrap();
        
        // Process some jobs first
        let _ = process_data(Context(ctx.clone()), TangleArgs1("test".to_string())).await.unwrap();
        let _ = process_data(Context(ctx.clone()), TangleArgs1("test2".to_string())).await.unwrap();
        
        let stats_before = ctx.storage.get_stats().await;
        assert_eq!(stats_before.total_jobs, 2);
        
        // Reset counter
        let result = reset_counter(Context(ctx.clone())).await.unwrap();
        assert!(result.is_ok());
        
        let stats_after = ctx.storage.get_stats().await;
        assert_eq!(stats_after.total_jobs, 1); // Only the reset job itself
        assert_eq!(stats_after.job_counts_by_id[&RESET_COUNTER_JOB_ID], 1);
        assert!(!stats_after.job_counts_by_id.contains_key(&PROCESS_DATA_JOB_ID));
    }
}
