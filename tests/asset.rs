#![cfg(not(target_arch = "wasm32"))]

use anyhow::Result;
use bitmask_core::{
    create_asset, fund_wallet, get_vault, get_wallet_data, import_asset, save_mnemonic_seed,
    set_blinded_utxo,
};
use std::env;

const MNEMONIC: &str =
    "swing rose forest coral approve giggle public liar brave piano sound spirit";
const ENCRYPTION_PASSWORD: &str = "hunter2";
const SEED_PASSWORD: &str = "";

const TICKER: &str = "TEST";
const NAME: &str = "Test asset";
const PRECISION: u8 = 3;
const SUPPLY: u64 = 1000;

/// Test asset import
#[tokio::test]
async fn asset_import() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();

    // Import wallet
    let mnemonic_data = save_mnemonic_seed(
        MNEMONIC.to_owned(),
        ENCRYPTION_PASSWORD.to_owned(),
        SEED_PASSWORD.to_owned(),
    )?;

    let encrypted_descriptors = serde_json::to_string(&mnemonic_data.serialized_encrypted_message)?;

    // Get vault properties
    let vault = get_vault(ENCRYPTION_PASSWORD.to_owned(), encrypted_descriptors)?;

    // Get assets wallet data
    let btc_wallet =
        get_wallet_data(&vault.btc_descriptor, Some(&vault.btc_change_descriptor)).await?;

    // Get assets wallet data
    let assets_wallet = get_wallet_data(&vault.rgb_tokens_descriptor, None).await?;

    // Get UDAs wallet data
    let udas_wallet = get_wallet_data(&vault.rgb_nfts_descriptor, None).await?;

    // Fund vault
    let fund_vault_details = fund_wallet(
        &vault.btc_descriptor,
        &vault.btc_change_descriptor,
        &assets_wallet.address,
        &udas_wallet.address,
    )
    .await?;

    // Create a test asset
    let (genesis, _) = create_asset(
        TICKER,
        NAME,
        PRECISION,
        SUPPLY,
        fund_vault_details.send_assets,
    )?;

    let asset_id = genesis.contract_id().to_string();

    let asset = import_asset(&vault.rgb_tokens_descriptor, Some(&asset_id), None, None).await?;

    assert_eq!(asset.id, asset_id, "Asset IDs match");

    // Parse wallet data
    assert_eq!(
        btc_wallet.transactions,
        vec![],
        "list of transactions is empty"
    );

    set_blinded_utxo("0b199e9bbbb79a9a1bc8d9a59d0f02f9eef045c2923577e719739d2546f7296e:2")?;

    Ok(())
}
