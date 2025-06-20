use crate::{context::JobCounterContext, Result};
use blueprint_sdk::prelude::*;

pub const PROCESS_DATA_JOB_ID: u64 = 0;

#[debug_job]
pub async fn process_data(
    Context(ctx): Context<JobCounterContext>,
    TangleArgs1(data): TangleArgs1<String>,
) -> Result<TangleResult<String>> {
    let job_description = format!("Processing data: '{}'", data);
    
    // Simulate some processing work
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    
    // Increment the job counter and log
    let total_count = ctx.storage.increment_job_count(PROCESS_DATA_JOB_ID, job_description.clone()).await?;
    
    let result = format!("Processed '{}' - Total jobs processed: {}", data, total_count);
    
    Ok(TangleResult::ok(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::JobCounterContext;
    use blueprint_sdk::testing::TangleTestHarness;

    #[tokio::test]
    async fn test_process_data() {
        let harness = TangleTestHarness::new().await.unwrap();
        let ctx = JobCounterContext::new(harness.env()).await.unwrap();
        
        let result = process_data(
            Context(ctx.clone()),
            TangleArgs1("test data".to_string()),
        ).await.unwrap();
        
        assert!(result.is_ok());
        let stats = ctx.storage.get_stats().await;
        assert_eq!(stats.total_jobs, 1);
        assert_eq!(stats.job_counts_by_id[&PROCESS_DATA_JOB_ID], 1);
    }
}
