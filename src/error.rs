use std::fmt;
use std::result;
use ascii::FromAsciiError;
use failure::*;
use failure_derive::Fail;
use red_asn1;
use crate::messages::*;

pub type KerberosResult<T> = result::Result<T, KerberosError>;

#[derive(Debug)]
pub struct KerberosError {
    inner: Context<KerberosErrorKind>
}

#[derive(Clone, PartialEq, Debug, Fail)]
pub enum KerberosErrorKind {
    #[fail(display = "Invalid KDC hostname")]
    InvalidKDC,
    #[fail(display = "Network error")]
    NetworkError,
    #[fail(display = "Invalid ascii string")]
    InvalidAscii,
    #[fail(display = "Undefined type of principal name: {}", _0)]
    PrincipalNameTypeUndefined(String),
    #[fail(display = "Invalid microseconds value {}. Max is 999999", _0)]
    InvalidMicroseconds(u32),
    #[fail(display = "Not available data {}", _0)]
    NotAvailableData(String),
    #[fail (display = "Asn1 error: {}", _0)]
    Asn1Error(red_asn1::ErrorKind),
    #[fail (display = "Cryptography error: {}", _0)]
    CryptographyError(Box<KerberosCryptographyErrorKind>),
    #[fail (display = "Error resolving name: {}", _0)]
    NameResolutionError(String),
    #[fail (display = "Received KRB-ERROR response")]
    KrbErrorResponse(KrbError),
    #[fail (display = "Error parsing KdcRep: {}", _1)]
    ParseKdcRepError(KdcRep, Box<KerberosErrorKind>),
    #[fail (display = "None cipher algorithm supported was specified")]
    NoProvidedSupportedCipherAlgorithm,
    #[fail (display = "Error in i/o operation")]
    IOError,
    #[fail (display = "No key was provided")]
    NoKeyProvided,
}

#[derive(Clone, PartialEq, Debug, Fail)]
pub enum KerberosCryptographyErrorKind {
    #[fail (display = "Cipher algorithm with etype = {} is not supported", _0)]
    UnsupportedCipherAlgorithm(i32),
    #[fail (display = "Decryption error: {}", _0)]
    DecryptionError(String),
}

impl KerberosError {

    pub fn kind(&self) -> &KerberosErrorKind {
        return self.inner.get_context();
    }

}

impl Fail for KerberosError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for KerberosError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl From<KerberosErrorKind> for KerberosError {
    fn from(kind: KerberosErrorKind) -> KerberosError {
        return KerberosError {
            inner: Context::new(kind)
        };
    }
}

impl From<Context<KerberosErrorKind>> for KerberosError {
    fn from(inner: Context<KerberosErrorKind>) -> KerberosError {
        return KerberosError { inner };
    }
}

impl From<KerberosCryptographyErrorKind> for KerberosError {
    fn from(kind: KerberosCryptographyErrorKind) -> KerberosError {
        return KerberosError {
            inner: Context::new(
                KerberosErrorKind::CryptographyError(Box::new(kind))
            )
        };
    }
}


impl From<FromAsciiError<&str>> for KerberosError {
    fn from(_error: FromAsciiError<&str>) -> Self {
        return KerberosError {
            inner: Context::new(KerberosErrorKind::InvalidAscii)
        };
    }
}

impl From<red_asn1::Error> for KerberosError {
    fn from(error: red_asn1::Error) -> Self {
        return KerberosError {
            inner: Context::new(KerberosErrorKind::Asn1Error(error.kind().clone()))
        };
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_kerberos_error() {
        match produce_invalid_network_error() {
            Err(kerberos_error) => {
                match kerberos_error.kind() {
                    KerberosErrorKind::NetworkError  => {
                        
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
            _ => {
                unreachable!();
            }
        }
    }

    fn produce_invalid_network_error() -> KerberosResult<()> {
        Err(KerberosErrorKind::NetworkError)?;
        unreachable!();
    }
}