use once_cell::sync::OnceCell;
use server::server_client::ServerClient;
use server::{HealthRequest, SendRequest};
use tonic::metadata::AsciiMetadataValue;
use tonic::transport::Channel;
use tracing::info;

pub mod server {
    tonic::include_proto!("serverpb");
}

static CLIENT: OnceCell<ServerClient<Channel>> = OnceCell::new();
static APIKEY: OnceCell<AsciiMetadataValue> = OnceCell::new();

pub async fn init_client(end_point: String, auth_key: String) -> anyhow::Result<()> {
    let blzendpoint = end_point;

    let channel = Channel::from_shared(blzendpoint.to_string())?
        .connect()
        .await?;

    let apikeyvalue = AsciiMetadataValue::try_from(auth_key)?;
    APIKEY.set(apikeyvalue.clone()).ok();

    let mut client = ServerClient::new(channel);

    // send health check first
    let mut request = tonic::Request::new(HealthRequest {});
    request.metadata_mut().insert("apikey", apikeyvalue.clone());
    let response = client.get_health(request).await?;
    info!("get health response={:?}", response.into_inner().status);
    CLIENT.set(client).ok();
    Ok(())
}

pub async fn send(sig: String, tx_str: String) -> anyhow::Result<()> {
    let client = CLIENT.get().expect("Client not initialized").clone();

    let apikey = APIKEY.get().unwrap().clone();

    let mut tx_request = tonic::Request::new(SendRequest {
        transaction: tx_str,
        mode: "fast".to_string(),
        safe_window: None,
        revert_protection: false,
    });
    tx_request.metadata_mut().insert("apikey", apikey.clone());

    info!("send grpc tx begin: {:?}", sig);
    let response = client.clone().send_transaction(tx_request).await?;
    info!("send grpc tx end: {:?}", response);
    Ok(())
}
