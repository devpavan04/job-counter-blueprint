use job_counter_lib::{
    context::JobCounterContext,
    jobs::{
        process_data::process_data,
        get_stats::get_stats,
        reset_counter::reset_counter,
        PROCESS_DATA_JOB_ID,
        GET_STATS_JOB_ID,
        RESET_COUNTER_JOB_ID,
    },
};
use blueprint_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<(), sdk::Error> {
    tracing_subscriber::init();
    
    let env = BlueprintEnvironment::load()?;

    let signer = env.keystore().first_local::<SpSr25519>()?;
    let pair = env.keystore().get_secret::<SpSr25519>(&signer)?;
    let signer = TanglePairSigner::new(pair.0);

    let client = env.tangle_client().await?;
    let producer = TangleProducer::finalized_blocks(client.rpc_client.clone()).await?;
    let consumer = TangleConsumer::new(client.rpc_client.clone(), signer);
    let context = JobCounterContext::new(env.clone()).await?;

    BlueprintRunner::builder(TangleConfig::default(), env)
        .router(
            Router::new()
                .route(PROCESS_DATA_JOB_ID, process_data.layer(TangleLayer))
                .route(GET_STATS_JOB_ID, get_stats.layer(TangleLayer))
                .route(RESET_COUNTER_JOB_ID, reset_counter.layer(TangleLayer))
                .with_context(context),
        )
        .producer(producer)
        .consumer(consumer)
        .run()
        .await
}
