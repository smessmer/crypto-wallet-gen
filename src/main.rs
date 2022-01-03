use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    crypto_wallet_gen::cli_main().await?;

    Ok(())
}
