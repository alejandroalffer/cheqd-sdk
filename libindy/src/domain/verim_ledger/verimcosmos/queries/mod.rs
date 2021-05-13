// TODO: Queries
// pub struct QueryGetNymRequest {
//     #[prost(uint64, tag = "1")]
//     pub id: u64,
// }
//
// pub struct QueryGetNymResponse {
//     #[prost(message, optional, tag = "1")]
//     pub nym: ::core::option::Option<Nym>,
// }
//
// pub struct QueryAllNymRequest {
//     pub pagination:
//         ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
// }
//
// pub struct QueryAllNymResponse {
//     pub nym: ::prost::alloc::vec::Vec<Nym>,
//     pub pagination:
//         ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
// }
mod query_get_nym_request;
mod query_get_nym_response;

pub use query_get_nym_request::QueryGetNymRequest;
pub use query_get_nym_response::QueryGetNymResponse;