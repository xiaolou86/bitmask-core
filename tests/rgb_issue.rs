#![cfg(not(target_arch = "wasm32"))]
use anyhow::Result;
use bitmask_core::operations::rgb::issue::issue_contract;
use rgbstd::persistence::Stock;

mod rgb_test_utils;
use rgb_test_utils::DumbResolve;

#[tokio::test]
async fn issue_contract_test() -> Result<()> {
    let ticker = "DIBA1";
    let name = "DIBA1";
    let description =
        "1 2 3 testing... 1 2 3 testing... 1 2 3 testing... 1 2 3 testing.... 1 2 3 testing";
    let precision = 8;
    let supply = 10;
    let iface = "RGB20";
    let seal = "tapret1st:70339a6b27f55105da2d050babc759f046c21c26b7b75e9394bc1d818e50ff52:0";

    let mut stock = Stock::default();
    let resolver = DumbResolve {};

    let contract = issue_contract(
        ticker,
        name,
        description,
        precision,
        supply,
        iface,
        seal,
        resolver,
        &mut stock,
    );

    assert!(contract.is_ok());

    Ok(())
}
