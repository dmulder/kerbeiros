use dns_lookup;
use std::net::IpAddr;
use crate::error::*;
use ascii::AsciiString;

pub fn resolve_realm_kdc(realm: &AsciiString) -> Result<IpAddr> {
    let ips = dns_lookup::lookup_host(&realm.to_string()).map_err(|_|
        ErrorKind::NameResolutionError(realm.to_string())
    )?;

    if ips.len() == 0 {
        return Err(ErrorKind::NameResolutionError(realm.to_string()))?;
    }

    return Ok(ips[0]);
}



