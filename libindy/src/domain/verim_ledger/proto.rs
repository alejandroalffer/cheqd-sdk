use cosmos_sdk::tx::MsgProto;

pub mod verimid {
    pub mod verimnode {
        pub mod verim {
            include!(concat!(
            env!("OUT_DIR"),
            "/prost/verimid.verimnode.verim.rs"
            ));
        }
    }
}

impl MsgProto for verimid::verimnode::verim::MsgCreateNym {
    const TYPE_URL: &'static str = "/verimid.verimnode.verim.MsgCreateNym";
}

impl MsgProto for verimid::verimnode::verim::MsgUpdateNym {
    const TYPE_URL: &'static str = "/verimid.verimnode.verim.MsgUpdateNym";
}

impl MsgProto for verimid::verimnode::verim::MsgDeleteNym {
    const TYPE_URL: &'static str = "/verimid.verimnode.verim.MsgDeleteNym";
}
