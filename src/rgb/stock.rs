use amplify::confinement::Confined;
use anyhow::Result;
use rgbstd::persistence::Stock;
use strict_encoding::{StrictDeserialize, StrictSerialize};

use crate::carbonado::{retrieve, store};

pub async fn store_stock(sk: &str, name: &str, stock: &Stock) -> Result<()> {
    let data = stock
        .to_strict_serialized::<{ usize::MAX }>()
        .expect("")
        .to_vec();

    store(sk, name, &data).await
}

pub async fn retrieve_stock(sk: &str, name: &str) -> Result<Stock> {
    let data = retrieve(sk, name).await.unwrap_or_default();
    if data.is_empty() {
        Ok(Stock::default())
    } else {
        let confined = Confined::try_from_iter(data.iter().copied()).expect("");
        let stock = Stock::from_strict_serialized::<{ usize::MAX }>(confined).expect("");

        Ok(stock)
    }
}
