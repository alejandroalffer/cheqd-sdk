use std::fmt::Debug;

use cosmos_ext::ProstMessageExt;
use cosmos_sdk::proto::cosmos::tx::v1beta1::{SignDoc as ProtoSignDoc, TxRaw};
use cosmos_sdk::tx::{Msg, MsgProto, MsgType, Raw, SignDoc};
use indy_api_types::errors::IndyResult;
use prost::Message;
use prost_types::Any;

pub mod cosmos_ext;
pub mod proto;
pub mod verimcosmos;
pub mod cosmos;

pub trait VerimProto: Eq + Debug {
    type Proto: prost::Message;

    fn to_proto(&self) -> Self::Proto;
    fn from_proto(proto: &Self::Proto) -> Self;
}
