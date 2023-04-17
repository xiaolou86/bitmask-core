use std::str::FromStr;

use bitcoin::{EcdsaSighashType, OutPoint};
use bitcoin_blockchain::locks::{LockTime, SeqNo};
use bitcoin_scripts::PubkeyScript;
use miniscript_crate::Descriptor;
use psbt::ProprietaryKeyType;
use wallet::psbt::Psbt;
use wallet::{
    descriptors::InputDescriptor,
    hd::{DerivationAccount, UnhardenedIndex},
    onchain::ResolveTx,
    psbt::{ProprietaryKeyDescriptor, ProprietaryKeyError, ProprietaryKeyLocation},
};

use super::constants::RGB_PSBT_TAPRET;
use super::structs::AddressAmount;

pub fn create_psbt(
    descriptor_pub: String,
    asset_utxo: String,
    asset_utxo_terminal: String,
    change_index: Option<String>,
    bitcoin_changes: Vec<String>,
    fee: u64,
    tx_resolver: &impl ResolveTx,
) -> Result<Psbt, ProprietaryKeyError> {
    let outpoint: OutPoint = asset_utxo.parse().expect("");
    let inputs = vec![InputDescriptor {
        outpoint,
        terminal: asset_utxo_terminal.parse().expect(""),
        seq_no: SeqNo::default(),
        tweak: None,
        sighash_type: EcdsaSighashType::All,
    }];

    let bitcoin_addresses: Vec<AddressAmount> = bitcoin_changes
        .into_iter()
        .map(|btc| AddressAmount::from_str(btc.as_str()).expect("invalid AddressFormat parse"))
        .collect();

    let outputs: Vec<(PubkeyScript, u64)> = bitcoin_addresses
        .into_iter()
        .map(|AddressAmount { address, amount }| (address.script_pubkey().into(), amount))
        .collect();

    let descriptor: &Descriptor<DerivationAccount> =
        &Descriptor::from_str(&descriptor_pub).expect("");
    let proprietary_keys = vec![ProprietaryKeyDescriptor {
        // TODO: Review that after amount protocol
        location: ProprietaryKeyLocation::Output(0_u16),
        ty: ProprietaryKeyType {
            prefix: RGB_PSBT_TAPRET.to_owned(),
            subtype: outpoint.vout as u8,
        },
        key: None,
        value: None,
    }];

    let lock_time = LockTime::anytime();
    let change_index = match change_index {
        Some(index) => UnhardenedIndex::from_str(index.as_str()).expect(""),
        _ => UnhardenedIndex::default(),
    };

    let mut psbt = Psbt::construct(
        descriptor,
        &inputs,
        &outputs,
        change_index,
        fee,
        tx_resolver,
    )
    .expect("cannot be construct PSBT information");

    psbt.fallback_locktime = Some(lock_time);

    for key in proprietary_keys {
        match key.location {
            ProprietaryKeyLocation::Input(pos) if pos as usize >= psbt.inputs.len() => {
                return Err(ProprietaryKeyError::InputOutOfRange(pos, psbt.inputs.len()))
            }
            ProprietaryKeyLocation::Output(pos) if pos as usize >= psbt.outputs.len() => {
                return Err(ProprietaryKeyError::OutputOutOfRange(
                    pos,
                    psbt.inputs.len(),
                ))
            }
            ProprietaryKeyLocation::Global => {
                psbt.proprietary.insert(
                    key.clone().into(),
                    key.value.as_ref().cloned().unwrap_or_default(),
                );
            }
            ProprietaryKeyLocation::Input(pos) => {
                psbt.inputs[pos as usize].proprietary.insert(
                    key.clone().into(),
                    key.value.as_ref().cloned().unwrap_or_default(),
                );
            }
            ProprietaryKeyLocation::Output(pos) => {
                psbt.outputs[pos as usize].proprietary.insert(
                    key.clone().into(),
                    key.value.as_ref().cloned().unwrap_or_default(),
                );
            }
        }
    }

    Ok(psbt)
}
