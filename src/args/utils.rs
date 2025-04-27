use std::env;

use super::Args;
use anyhow::Result;
use log::info;

use tokio_stream::StreamExt;
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient, Interceptor};
use yellowstone_grpc_proto::geyser::CommitmentLevel;

impl Args {
    pub async fn connect(self) -> Result<GeyserGrpcClient<impl Interceptor>> {
        let tls_config = ClientTlsConfig::new().with_native_roots();
        let endpoint = match &self.endpoint {
            Some(endpoint) => endpoint.clone(),
            None => env::var("GRPC_ENDPOINT").unwrap(),
        };
        let builder =
            GeyserGrpcClient::build_from_shared(endpoint.clone())?.tls_config(tls_config)?;

        builder
            .connect()
            .await
            .map_err(|e| anyhow::anyhow!("failed to connect: {}", e))
    }
    pub async fn greyser_health_watch(
        &self,
        client: &mut GeyserGrpcClient<impl Interceptor>,
    ) -> Result<()> {
        let mut stream = client.health_watch().await?;
        info!("health_watch stream started...");
        while let Some(health) = stream.next().await {
            info!("health message: {:?}", health?);
        }
        info!("health_watch stream ended...");
        Ok(())
    }
    pub async fn server_version(
        &self,
        client: &mut GeyserGrpcClient<impl Interceptor>,
    ) -> Result<()> {
        let version = client.get_version().await?;
        pretty_print_json(&version.version, "Version response")?;
        Ok(())
    }
    pub async fn get_latest_blockhash(
        &self,
        client: &mut GeyserGrpcClient<impl Interceptor>,
    ) -> Result<()> {
        let blockhash = client
            .get_latest_blockhash(Some(CommitmentLevel::Processed))
            .await?;
        info!("latest_blockhash: {:#?}", blockhash);
        Ok(())
    }
}

pub fn pretty_print_json(input: &str, prefix: &str) -> Result<()> {
    let s: serde_json::Value = serde_json::from_str(input)?;
    println!("{}: {}", prefix, serde_json::to_string_pretty(&s)?);
    Ok(())
}
