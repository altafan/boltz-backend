use crate::ark::ArkClient;
use crate::wallet::Wallet;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use bitcoin::{bip32::Xpriv, secp256k1};
use std::sync::Arc;

pub struct Ark {
    // Kept for when key derivation is wired to Fulmine's GetPubKey call
    #[allow(dead_code)]
    client: Arc<ArkClient>,
}

impl Ark {
    pub fn new(client: Arc<ArkClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Wallet for Ark {
    fn decode_address(&self, _address: &str) -> Result<Vec<u8>> {
        Err(anyhow!("not implemented for ark"))
    }

    fn derive_keys(&self, _index: u64) -> Result<Xpriv> {
        Err(anyhow!("not implemented for ark"))
    }

    // Since Fulmine v0.4 the keys of vHTLCs are derived from an HD wallet. Resolving an
    // index to a public key requires the GetPubKey call, which this sync trait cannot do.
    fn derive_pubkey(
        &self,
        _secp: &secp256k1::Secp256k1<secp256k1::All>,
        _index: u64,
    ) -> Result<secp256k1::PublicKey> {
        Err(anyhow!("not implemented for ark"))
    }

    fn derive_blinding_key(&self, _script_pubkey: Vec<u8>) -> Result<Vec<u8>> {
        Err(anyhow!("not implemented for ark"))
    }

    async fn get_address(&self, _wallet: Option<&str>, _label: &str) -> Result<String> {
        Err(anyhow!("not implemented for ark"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ark::get_client;
    use crate::chain::BaseClient;

    #[tokio::test]
    async fn test_derive_pubkey() {
        let mut client = get_client().await;
        client.connect().await.unwrap();
        let wallet = Ark::new(Arc::new(client.clone()));

        assert_eq!(
            wallet
                .derive_pubkey(&secp256k1::Secp256k1::new(), 0)
                .err()
                .unwrap()
                .to_string(),
            "not implemented for ark"
        );
    }
}
