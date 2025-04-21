#[tokio::main]

async fn main() -> anyhow::Result<()> {
    verifier::run().await
}
