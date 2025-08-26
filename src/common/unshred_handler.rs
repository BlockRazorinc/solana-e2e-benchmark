use std::sync::{Arc, RwLock};
use tracing::info;
use unshred::{TransactionEvent, TransactionHandler};

pub struct MyShredHandler {
    pub signature: RwLock<String>,
}

pub struct SharedHandler(pub Arc<MyShredHandler>);

impl TransactionHandler for SharedHandler {
    fn handle_transaction(&self, event: &TransactionEvent) -> anyhow::Result<()> {
        self.0.handle_transaction(event)
    }
}

impl TransactionHandler for MyShredHandler {
    fn handle_transaction(&self, event: &TransactionEvent) -> anyhow::Result<()> {
        let signature = solana_sdk::bs58::encode(&event.transaction.signatures[0]).into_string();
        if signature == *self.signature.read().unwrap() {
            info!("UnShred received: {:?}", signature);
        }
        Ok(())
    }
}

impl MyShredHandler {
    pub fn set_signature(&self, signature: String) {
        info!("changed before: {}", *self.signature.read().unwrap());
        *self.signature.write().unwrap() = signature;
        info!("changed after: {}", *self.signature.read().unwrap());
    }

    pub fn get_signature(&self) -> String {
        self.signature.read().unwrap().clone()
    }
}
