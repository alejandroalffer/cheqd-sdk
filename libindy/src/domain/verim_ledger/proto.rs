use cosmos_sdk::tx::MsgProto;

pub mod verimid {
    pub mod verimcosmos {
        pub mod verimcosmos {
            include!(concat!(
                env!("OUT_DIR"),
                "/prost/verimid.verimnode.verim.rs"
            ));
        }
    }
}

impl MsgProto for verimid::verimcosmos::verimcosmos::MsgCreateNym {
    const TYPE_URL: &'static str = "/verimid.verimnode.verim.MsgCreateNym";
}

impl MsgProto for verimid::verimcosmos::verimcosmos::MsgUpdateNym {
    const TYPE_URL: &'static str = "/verimid.verimnode.verim.MsgUpdateNym";
}

impl MsgProto for verimid::verimcosmos::verimcosmos::MsgDeleteNym {
    const TYPE_URL: &'static str = "/verimid.verimnode.verim.MsgDeleteNym";
}
