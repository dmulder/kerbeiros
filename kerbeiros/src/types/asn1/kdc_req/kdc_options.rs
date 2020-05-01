use super::super::basics::kerberos_flags::*;

/// (*KDCOptions*) Options used in Kerberos requests.
pub type KdcOptions = KerberosFlags;
pub(crate) type KdcOptionsAsn1 = KerberosFlagsAsn1;

#[cfg(test)]
mod tests {
    use super::*;
    use kerberos_constants::kdc_options::*;
    use red_asn1::*;

    #[test]
    fn test_set_option() {
        let mut kdc_options = KdcOptions::default();
        kdc_options.set_flags(FORWARDABLE);

        assert!(kdc_options.has_flag(FORWARDABLE));
    }

    #[test]
    fn test_delete_option() {
        let mut kdc_options = KdcOptions::default();
        kdc_options.set_flags(PROXIABLE);
        kdc_options.del_flags(PROXIABLE);

        assert!(!kdc_options.has_flag(PROXIABLE));
    }

    #[test]
    fn test_set_flags() {
        let mut kdc_options = KdcOptions::default();
        kdc_options.set_flags(FORWARDABLE | POSTDATED);

        assert!(kdc_options.has_flag(FORWARDABLE));
        assert!(kdc_options.has_flag(POSTDATED));
    }

    #[test]
    fn test_delete_options() {
        let mut kdc_options = KdcOptions::default();
        kdc_options.set_flags(FORWARDABLE | POSTDATED);
        kdc_options.del_flags(FORWARDABLE | POSTDATED);

        assert!(!kdc_options.has_flag(FORWARDABLE));
        assert!(!kdc_options.has_flag(POSTDATED));
    }

    #[test]
    fn test_delete_one_options() {
        let mut kdc_options = KdcOptions::default();
        kdc_options.set_flags(FORWARDABLE | POSTDATED);
        kdc_options.del_flags(FORWARDABLE);

        assert!(!kdc_options.has_flag(FORWARDABLE));
        assert!(kdc_options.has_flag(POSTDATED));
    }

    #[test]
    fn test_encode_kdcoptions() {
        let mut kdc_options = KdcOptions::default();
        kdc_options.set_flags(FORWARDABLE | RENEWABLE | CANONICALIZE | RENEWABLE_OK);
        assert_eq!(
            vec![0x03, 0x05, 0x0, 0x40, 0x81, 0x00, 0x10],
            KdcOptionsAsn1::from(&kdc_options).encode().unwrap()
        )
    }
}
