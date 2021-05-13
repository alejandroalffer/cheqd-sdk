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

impl MsgProto for verimid::verimcosmos::verimcosmos::QueryGetNymRequest {
    const TYPE_URL: &'static str = "/verimid.verimcosmos.verimcosmos.QueryGetNymRequest";
}

impl MsgProto for verimid::verimcosmos::verimcosmos::QueryGetNymResponse {
    const TYPE_URL: &'static str = "/verimid.verimcosmos.verimcosmos.QueryGetNymResponse";
}

impl MsgProto for verimid::verimcosmos::verimcosmos::Nym {
    const TYPE_URL: &'static str = "/verimid.verimcosmos.verimcosmos.Nym";
}
