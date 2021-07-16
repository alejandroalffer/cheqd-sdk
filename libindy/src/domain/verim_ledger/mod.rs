use std::fmt::Debug;

use indy_api_types::errors::IndyResult;

use prost_ext::ProstMessageExt;

pub mod cosmos_ext;
pub mod prost_ext;
pub mod proto;
pub mod verim;
pub mod auth;
pub mod base;
pub mod crypto;

pub trait VerimProto: Eq + Debug + Sized {
    type Proto: prost::Message + Default;

    fn to_proto(&self) -> Self::Proto;
    fn from_proto(proto: &Self::Proto) -> IndyResult<Self>;

    fn to_proto_bytes(&self) -> IndyResult<Vec<u8>> {
        Ok(self.to_proto().to_bytes()?)
    }

    fn from_proto_bytes(bytes: &[u8]) -> IndyResult<Self> {
        let proto = Self::Proto::from_bytes(bytes)?;
        Ok(Self::from_proto(&proto)?)
    }
}
