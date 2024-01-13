use std::env;
use dotenv::from_filename;
use bdk::{Wallet, SyncOptions};
use bdk::bitcoin::Network;
use bdk::database::MemoryDatabase;
use bdk::electrum_client::Client;
use bdk::blockchain::ElectrumBlockchain;


fn main() -> anyhow::Result<()> {

    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let blockchain = ElectrumBlockchain::from(client);

    from_filename(".env").ok();


    let wallet_descriptor = env::var("WALLET_DESCRIPTOR")?;

    
    let wallet = Wallet::new(
        &wallet_descriptor.clone(),
        None,
        Network::Testnet,
        MemoryDatabase::default(),
    )?;

    wallet.sync(&blockchain, SyncOptions::default())?;

    let address = wallet.get_address(bdk::wallet::AddressIndex::New)?;
    
    println!("Descriptor balance: {} SAT", wallet.get_balance()?);
    println!("Current address: {}", address);

    Ok(())
}
