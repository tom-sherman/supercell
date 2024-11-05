use std::collections::HashSet;

use anyhow::{anyhow, Result};
use chrono::Duration;
use serde::Deserialize;
use tokio::time::{sleep, Instant};
use tokio_util::sync::CancellationToken;

use crate::storage::{verifcation_method_insert, verification_method_cleanup, StoragePool};

#[derive(Deserialize)]
struct VerificationMethod {
    #[serde(rename = "publicKeyMultibase")]
    public_key_multibase: String,
}

#[derive(Deserialize)]
struct ResolvedPlcDid {
    id: String,
    #[serde(rename = "verificationMethod")]
    verification_method: Vec<VerificationMethod>,
}

pub struct VerificationMethodCacheTask {
    pool: StoragePool,
    http_client: reqwest::Client,
    plc_hostname: String,
    dids: HashSet<String>,
    cancellation_token: CancellationToken,
}

impl VerificationMethodCacheTask {
    pub fn new(
        pool: StoragePool,
        http_client: reqwest::Client,
        plc_hostname: String,
        dids: HashSet<String>,
        cancellation_token: CancellationToken,
    ) -> Self {
        Self {
            pool,
            http_client,
            plc_hostname,
            dids,
            cancellation_token,
        }
    }

    pub async fn run_background(&self, interval: Duration) -> Result<()> {
        let interval = interval.to_std()?;

        let sleeper = sleep(interval);
        tokio::pin!(sleeper);

        loop {
            tokio::select! {
            () = self.cancellation_token.cancelled() => {
                break;
            },
            () = &mut sleeper => {

                    if let Err(err) = self.main().await {
                        tracing::error!("StatsTask task failed: {}", err);
                    }


                sleeper.as_mut().reset(Instant::now() + interval);
            }
            }
        }
        Ok(())
    }

    pub async fn main(&self) -> Result<()> {
        for did in &self.dids {
            let query_response = self.plc_query(did).await;
            if let Err(err) = query_response {
                tracing::error!(error = ?err, "Failed to query PLC for DID: {}", did);
                continue;
            }
            let key = query_response.unwrap();

            verifcation_method_insert(&self.pool, did, &key).await?;
        }

        verification_method_cleanup(&self.pool).await?;
        Ok(())
    }

    async fn plc_query(&self, did: &str) -> Result<String> {
        let url = if let Some(hostname) = did.strip_prefix("did:web:") {
            format!("https://{}/.well-known/did.json", hostname)
        } else {
            format!("https://{}/{}", self.plc_hostname, did)
        };

        let resolved_did: ResolvedPlcDid = self
            .http_client
            .get(url)
            .timeout(Duration::seconds(10).to_std()?)
            .send()
            .await?
            .json()
            .await?;

        if resolved_did.id != did {
            return Err(anyhow!("DID mismatch"));
        }

        let key = resolved_did
            .verification_method
            .first()
            .map(|value| value.public_key_multibase.clone());

        key.ok_or(anyhow!("No key found"))
    }
}
