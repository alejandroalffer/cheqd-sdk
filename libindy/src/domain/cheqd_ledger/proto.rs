use cosmos_sdk::tx::MsgProto;

pub mod cheqdid {
    pub mod cheqdnode {
        pub mod cheqd {
            include!(concat!(
            env!("OUT_DIR"),
            "/prost/cheqdid.cheqdnode.cheqd.rs"
            ));
        }
    }
}

impl MsgProto for cheqdid::cheqdnode::cheqd::MsgCreateNym {
    const TYPE_URL: &'static str = "/cheqdid.cheqdnode.cheqd.MsgCreateNym";
}

impl MsgProto for cheqdid::cheqdnode::cheqd::MsgUpdateNym {
    const TYPE_URL: &'static str = "/cheqdid.cheqdnode.cheqd.MsgUpdateNym";
}

impl MsgProto for cheqdid::cheqdnode::cheqd::MsgDeleteNym {
    const TYPE_URL: &'static str = "/cheqdid.cheqdnode.cheqd.MsgDeleteNym";
}
