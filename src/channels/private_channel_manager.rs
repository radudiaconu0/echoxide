// use crate::channels::channel_manager::ChannelManager;
// use crate::channels::public_channel_manager::{JoinResponse, LeaveResponse};
// use crate::message::PusherMessage;
// use crate::{message, utils};
// use fastwebsockets::{FragmentCollector, WebSocket};
//
// struct PrivateChannelManager<S> {
// }
//
// impl<S> PrivateChannelManager<S> {
//     pub fn new() -> Self {
//         Self {
//
//         }
//     }
// }
//
// impl<S> ChannelManager<S> for PrivateChannelManager<S> {
//     fn join(
//         &self,
//         ws: FragmentCollector<S>,
//         channel: &str,
//         message: message::PusherMessage,
//     ) -> JoinResponse<S> {
//         let passed_signature = match message.data {
//             Some(mut data) => {
//                 let auth = data.extra.get("auth").unwrap().to_string();
//                 auth
//             }
//             _ => "".to_string(),
//         };
//         JoinResponse {
//             server: ws,
//             success: false,
//             channel_connections: None,
//             auth_error: None,
//             member: None,
//             error_message: None,
//             error_code: None,
//             type_: None,
//         }
//     }
//
//     fn leave(&self, channel: &str) -> LeaveResponse {
//         LeaveResponse {
//             left: false,
//             remaining_connections: None,
//             member: None,
//         }
//     }
// }
