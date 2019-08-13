//! Classes that represents the Kerberos credentials
//! 
//! Each credential if composed by a ticket and information related to the Kerberos session, such as client name, realm name or session key.
//! 

mod file;

mod credential;
pub use credential::*;

mod credentialwarehouse;
pub use credentialwarehouse::*;

mod mappers;
pub(crate) use mappers::CredentialKrbInfoMapper;