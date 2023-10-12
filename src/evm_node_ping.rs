use ethers::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_server = "https://avalanche-fuji-c-chain.publicnode.com";
    let provider = Provider::<Http>::try_from(rpc_server)?;
    let block_number = provider.get_block_number().await?;
    println!("Block number of current block {}", block_number);
    let previous_block_number = block_number - 1;

    println!("Core-eth Version {:?}", provider.client_version().await?);
    println!("Chain Id {:?}", provider.get_chainid().await?);

    println!(
        "Uncle count for the block {:?}",
        provider.get_uncle_count(block_number).await?
    );
    println!(
        "Block info for block number {} \n{:?}",
        previous_block_number,
        provider.get_block(previous_block_number).await?.unwrap()
    );

    println!("Node information \n{:?}", provider.provider());

    // Return result
    Ok(())
}
