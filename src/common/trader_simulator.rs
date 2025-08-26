use crate::common::unshred_handler::MyShredHandler;
use crate::common::{tx_constructor, tx_sender};
use std::sync::Arc;
use tracing::info;

pub async fn one_e2e_test(handler: Arc<MyShredHandler>) -> anyhow::Result<()> {
    info!("Begin construct transaction");
    let (sig, tx_str) = tx_constructor::construct_tx().await;
    info!("End construct transaction signature is : {}", sig);
    handler.set_signature(sig.clone());
    info!("Handler Get Signature: {}", handler.get_signature());
    if let Err(e) = tx_sender::send(sig, tx_str).await {
        eprintln!("send tx error: {}", e);
    }
    Ok(())
}

pub async fn one_client_test(sig: String, tx_str: String) -> anyhow::Result<()> {
    tx_sender::send(sig, tx_str).await.map_err(|e| {
        eprintln!("send tx error: {}", e);
        anyhow::Error::msg(e.to_string())
    })?;
    Ok(())
}
