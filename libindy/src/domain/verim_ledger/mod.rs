use std::fmt::Debug;

use cosmos_sdk::proto::cosmos::tx::v1beta1::{SignDoc as ProtoSignDoc, TxRaw};
use cosmos_sdk::tx::{Msg, MsgProto, MsgType, Raw, SignDoc};
use indy_api_types::errors::IndyResult;
use prost::Message;
use prost_ext::ProstMessageExt;
use prost_types::Any;

pub mod cosmos;
pub mod cosmos_ext;
pub mod prost_ext;
pub mod proto;
pub mod verim;

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
