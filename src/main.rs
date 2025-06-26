    use bluest::Adapter;
    use futures::stream::StreamExt;
    // use tokio::runtime::Runtime; // Use this for nested async and if #[tokio::main] doesn't suit your need

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> { 
        // `Result` is an enum; we use a Box - trait object for flexibility in error types
        let adapter = Adapter::default().await.ok_or("Bluetooth adapter not found!")?;
        adapter.wait_available().await?; // Wait untill ready

        println!("Starting scan...");
        let mut scan = adapter.scan(&[]).await?;
        println!("Scan started");

        while let Some(discovered_device) = scan.next().await {
            let device = discovered_device.device;
            println!(
                "Device: {} ({:?})", // Dispay and Debug  
                device.name().unwrap_or("(unknown)".to_string()), 
                device.id()
            );
        }
        Ok(())
}