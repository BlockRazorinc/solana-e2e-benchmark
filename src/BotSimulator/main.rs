use anyhow::Result;
use clap::Parser;
use solanabench::common::types::RunArgs;
use solanabench::common::unshred_handler::{MyShredHandler, SharedHandler};
use solanabench::common::{trader_simulator, tx_sender, yellow_stone_grpc_client};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tracing::info;
use unshred::UnshredProcessor;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let run_args = RunArgs::parse();
    let handler = Arc::new(MyShredHandler {
        signature: RwLock::new("".to_string()),
    });
    let shred_handler = SharedHandler(handler.clone());
    let handler_clone = handler.clone();
    tokio::spawn(async move {
        yellow_stone_grpc_client::grpc_subscribe(
            run_args.yellow_stone_grpc_endpoint,
            run_args.concerned_account,
        )
        .await
    });

    let sender_grpc_endpoint = run_args.sender_grpc_endpoint;
    let sender_grpc_auth_key = run_args.sender_grpc_auth_key;
    let _ = tx_sender::init_client(sender_grpc_endpoint, sender_grpc_auth_key).await;

    tokio::spawn(async move {
        let processor = UnshredProcessor::builder()
            .handler(shred_handler)
            .bind_address(run_args.shred_listen_addr)
            .build()
            .unwrap();
        processor.run().await.unwrap();
    });

    info!("wait 5 seconds for stream ready");
    tokio::time::sleep(Duration::from_secs(10)).await;
    for i in 0..100 {
        info!(
            "wait for 5 seconds to send tx, current test case: {}",
            i
        );
        let _ = trader_simulator::one_e2e_test(handler_clone.clone()).await;
        tokio::time::sleep(Duration::from_secs(5)).await;
    }

    info!("test finished");
    tokio::time::sleep(Duration::from_secs(60)).await;
    Ok(())
}
