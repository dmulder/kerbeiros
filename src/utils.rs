//! Implement functions that can be useful to support the main library functionality.

use dns_lookup;
use std::net::IpAddr;
use crate::error::*;
use ascii::AsciiString;



/// Resolve the address of the KDC from the name of the realm.
/// 
/// # Errors
/// Returns [`Error`](../error/struct.Error.html) if it is not possible to resolve the domain name or the resolution does not include any IP address.
pub fn resolve_realm_kdc(realm: &AsciiString) -> Result<IpAddr> {
    let ips = dns_lookup::lookup_host(&realm.to_string()).map_err(|_|
        ErrorKind::NameResolutionError(realm.to_string())
    )?;

    if ips.len() == 0 {
        return Err(ErrorKind::NameResolutionError(realm.to_string()))?;
    }

    return Ok(ips[0]);
}



