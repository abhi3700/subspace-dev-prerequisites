//! Convert Substrate address (SS58) to Subspace's address format with prefix 2254
//! Tool: https://polkadot.subscan.io/tools/format_transform
//!
//! SS58 is a simple account format designed for Substrate based chains.
//! There's no problem with using other account formats for a chain,
//! but this serves as a robust default. It is heavily based on Bitcoin's
//! Base-58-check format with a few alterations.

use sp_core::crypto::{Ss58AddressFormat, Ss58Codec};
use sp_core::sr25519::Public;

fn convert_address(address: &str, new_format: Ss58AddressFormat) -> Result<String, &'static str> {
    // check & decode the public key from address as per base-58 (as used in Bitcoin) i.e. SS58.
    let public_key = match Public::from_ss58check_with_version(address) {
        Ok((public_key, _)) => public_key,
        Err(_) => return Err("Invalid address"),
    };

    // convert from public key to address with new format as specified.
    Ok(public_key.to_ss58check_with_version(new_format))
}

fn main() {
    let address = "5DepcPH2mXTLeLubiF5kXdbtgdFKc53mt6J6Ne8ethqpVem4";
    let new_format = Ss58AddressFormat::custom(2254);
    match convert_address(address, new_format) {
        Ok(new_address) => println!("New address: {}", new_address),
        Err(err) => println!("Error: {}", err),
    }
}
