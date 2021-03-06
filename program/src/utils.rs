use std::cell::RefMut;

use bytemuck::{bytes_of, Contiguous};
use serum_dex::critbit::SlabView;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

pub fn gen_signer_seeds<'a>(nonce: &'a u64, acc_pk: &'a Pubkey) -> [&'a [u8]; 2] {
    [acc_pk.as_ref(), bytes_of(nonce)]
}


pub fn gen_signer_key(
    nonce: u64,
    acc_pk: &Pubkey,
    program_id: &Pubkey,
) -> Result<Pubkey, ProgramError> {
    let seeds = gen_signer_seeds(&nonce, acc_pk);
    Ok(Pubkey::create_program_address(&seeds, program_id)?)
}


pub fn create_signer_key_and_nonce(program_id: &Pubkey, acc_pk: &Pubkey) -> (Pubkey, u64) {
    for i in 0..=u64::MAX_VALUE {
        if let Ok(pk) = gen_signer_key(i, acc_pk, program_id) {
            return (pk, i);
        }
    }
    panic!("Could not generate signer key");
}


pub fn get_dex_best_price(slab: RefMut<serum_dex::critbit::Slab>, is_bid: bool) -> Option<u64> {
    if slab.is_empty() {
        None
    } else {
        if is_bid {
            let best_bid_h = slab.find_max().unwrap();
            let best_bid_px = slab.get(best_bid_h).unwrap().as_leaf().unwrap().price().get();
            Some(best_bid_px)
        } else {
            let best_ask_h = slab.find_min().unwrap();
            let best_ask_px = slab.get(best_ask_h).unwrap().as_leaf().unwrap().price().get();
            Some(best_ask_px)
        }
    }
}
