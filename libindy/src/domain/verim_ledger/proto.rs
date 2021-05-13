use cosmos_sdk::tx::MsgProto;

pub mod verimid {
    pub mod verimcosmos {
        pub mod verimcosmos {
            include!(concat!(
                env!("OUT_DIR"),
                "/prost/verimid.verimcosmos.verimcosmos.rs"
            ));
        }
    }
}

impl MsgProto for verimid::verimcosmos::verimcosmos::MsgCreateNym {
    const TYPE_URL: &'static str = "/verimid.verimcosmos.verimcosmos.MsgCreateNym";
}

impl MsgProto for verimid::verimcosmos::verimcosmos::MsgCreateNymResponse {
    const TYPE_URL: &'static str = "/verimid.verimcosmos.verimcosmos.MsgCreateNymResponse";
}

impl MsgProto for verimid::verimcosmos::verimcosmos::MsgUpdateNym {
    const TYPE_URL: &'static str = "/verimid.verimcosmos.verimcosmos.MsgUpdateNym";
}

impl MsgProto for verimid::verimcosmos::verimcosmos::MsgUpdateNymResponse {
    const TYPE_URL: &'static str = "/verimid.verimcosmos.verimcosmos.MsgUpdateNymResponse";
}

impl MsgProto for verimid::verimcosmos::verimcosmos::MsgDeleteNym {
    const TYPE_URL: &'static str = "/verimid.verimcosmos.verimcosmos.MsgDeleteNym";
}

impl MsgProto for verimid::verimcosmos::verimcosmos::MsgDeleteNymResponse {
    const TYPE_URL: &'static str = "/verimid.verimcosmos.verimcosmos.MsgDeleteNymResponse";
}