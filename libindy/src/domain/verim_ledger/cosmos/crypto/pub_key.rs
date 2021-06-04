use super::secp256k1;
use crate::domain::verim_ledger::VerimProto;
use crate::domain::verim_ledger::cosmos_ext::ProstMessageExt;

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
enum PubKey{
    Secp256k1(secp256k1::PubKey)
}

impl VerimProto for PubKey {
    type Proto = prost_types::Any;

    fn to_proto(&self) -> Self::Proto {
        unimplemented!()
    }

    fn from_proto(proto: &Self::Proto) -> Self {
        match proto.type_url.as_str() {
            "secp256k" => {
                let proto = <secp256k1::PubKey as VerimProto>::Proto::from_bytes(&proto.value.as_slice())?;
                PubKey::Secp256k1(secp256k1::PubKey::from_proto(&proto)) }
            None => None
        }
    }
}