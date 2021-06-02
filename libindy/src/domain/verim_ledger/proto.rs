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

impl MsgProto for verimid::verimcosmos::verimcosmos::QueryGetNymRequest {
    const TYPE_URL: &'static str = "/verimid.verimcosmos.verimcosmos.QueryGetNymRequest";
}

impl MsgProto for verimid::verimcosmos::verimcosmos::QueryGetNymResponse {
    const TYPE_URL: &'static str = "/verimid.verimcosmos.verimcosmos.QueryGetNymResponse";
}

impl MsgProto for verimid::verimcosmos::verimcosmos::QueryAllNymRequest {
    const TYPE_URL: &'static str = "/verimid.verimcosmos.verimcosmos.QueryAllNymRequest";
}

impl MsgProto for verimid::verimcosmos::verimcosmos::QueryAllNymResponse {
    const TYPE_URL: &'static str = "/verimid.verimcosmos.verimcosmos.QueryAllNymResponse";
}

impl MsgProto for verimid::verimcosmos::verimcosmos::Nym {
    const TYPE_URL: &'static str = "/verimid.verimcosmos.verimcosmos.Nym";
}

impl MsgProto for cosmos_sdk::proto::cosmos::base::query::v1beta1::PageRequest {
    const TYPE_URL: &'static str = "/cosmos_sdk.proto.cosmos.base.query.v1beta1.PageRequest";
}

impl MsgProto for cosmos_sdk::proto::cosmos::base::query::v1beta1::PageResponse {
    const TYPE_URL: &'static str = "/cosmos_sdk.proto.cosmos.base.query.v1beta1.PageResponse";
}