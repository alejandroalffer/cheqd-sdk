use std::fmt::Debug;

use cosmos_sdk::proto::cosmos::tx::v1beta1::{SignDoc as ProtoSignDoc, TxRaw};
use cosmos_sdk::tx::{Msg, MsgProto, MsgType, Raw, SignDoc};
use indy_api_types::errors::IndyResult;
use prost::Message;
use prost_types::Any;

pub mod cosmos_ext;
pub mod proto;
pub mod verimcosmos;

pub trait VerimMessage: Eq + Debug {
    type Proto: MsgType;

    fn to_proto(&self) -> Self::Proto;
    fn from_proto(proto: &Self::Proto) -> Self;

    fn to_msg(&self) -> IndyResult<Msg> {
        Ok(self.to_proto().to_msg()?)
    }

    fn from_msg(msg: &Msg) -> IndyResult<Self>
    where
        Self: Sized,
    {
        let proto = Self::Proto::from_msg(msg)?;
        Ok(Self::from_proto(&proto))
    }
}