use std::fmt::Debug;

use cosmos_sdk::{
    proto::cosmos::tx::v1beta1::{SignDoc as ProtoSignDoc, TxRaw},
    tx::{Msg, MsgProto, MsgType, Raw, SignDoc},
};
use cosmos_ext::ProstMessageExt;
use cosmos_sdk::tx::{Msg, MsgProto, MsgType, Raw, SignDoc};
use indy_api_types::errors::IndyResult;
use prost::Message;
use prost_types::Any;

mod cosmos;
pub mod cosmos_ext;
pub mod proto;
pub mod verimcosmos;

// TODO: Join with proto_struct
pub trait VerimMessage: Eq + Debug {
    type Proto: prost::Message + MsgType;

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

    // fn to_bytes(&self) -> IndyResult<Vec<u8>> {
    //     let proto = self.to_proto();
    //     Ok(proto.to_bytes()?)
    // }
    //
    // fn from_bytes(bytes: &[u8]) -> IndyResult<Self>
    // where
    //     Self: Sized,
    // {
    //     let proto = Self::Proto::from_bytes(bytes)?;
    //     Ok(Self::from_proto(&proto)?)
    // }
}

pub trait ProtoStruct {
    type Proto: Message;

    fn to_proto(&self) -> Self::Proto;
    fn from_proto(proto: &Self::Proto) -> Self;
}

pub trait ProtoEnum {
    type Proto;

    fn to_proto(&self) -> Self::Proto;
    fn from_proto(proto: &Self::Proto) -> Self;
}
