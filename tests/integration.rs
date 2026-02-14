use futures::stream;
use query_pipeline::api::RealApi; // Import the RealApi
use query_pipeline::{ingest_stream, spawn_processor};
use std::sync::Arc;

#[tokio::test]
async fn test_ingest_and_process() {
    let queries = vec!["one".into(), "two".into(), "one".into()];

    // Create the API implementation
    let api = Arc::new(RealApi);
    let processor_tx = spawn_processor(api);

    // Create query stream
    let query_stream = stream::iter(queries.clone());

    // Ingest items into the processor
    let ingest_result = ingest_stream(query_stream, processor_tx).await;

    // Give the processor time to complete
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    assert!(ingest_result.is_ok());
}
