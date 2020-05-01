use crate::error;
use crate::types::KerberosString;
use kerberos_ccache::CountedOctetString;

pub struct CountedOctetStringMapper {}

impl CountedOctetStringMapper {
    pub fn kerberos_string_to_counted_octet_string(
        kerberos_string: &KerberosString,
    ) -> CountedOctetString {
        return CountedOctetString::new(kerberos_string.as_bytes().to_vec())
    }

    pub fn counted_octet_string_to_kerberos_string(
        counted_octet_string: CountedOctetString,
    ) -> error::Result<KerberosString> {
        return Ok(KerberosString::from_ascii(counted_octet_string.data)?);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_kerberos_string_to_counted_octet_string() {
        let cos_string = CountedOctetStringMapper::kerberos_string_to_counted_octet_string(
            &KerberosString::from_ascii("ABC").unwrap()
        );
        
        assert_eq!(CountedOctetString::from("ABC"), cos_string);
    }
    
    #[test]
    fn test_counted_octet_string_to_kerberos_string() {
        let k_string: KerberosString = CountedOctetStringMapper::counted_octet_string_to_kerberos_string(
            CountedOctetString::from("ABC")
        ).unwrap();
        
        assert_eq!(KerberosString::from_ascii("ABC").unwrap(), k_string)
    }

    #[test]
    #[should_panic(expected = "InvalidAscii")]
    fn test_counted_octet_string_to_kerberos_string_fail() {
        CountedOctetStringMapper::counted_octet_string_to_kerberos_string(
            CountedOctetString::new(vec![0xff])
        ).unwrap();
    }

}
