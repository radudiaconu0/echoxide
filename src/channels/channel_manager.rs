use crate::channels::public_channel_manager::{JoinResponse, LeaveResponse};
use crate::message::PusherMessage;
use echoxide::WS;

pub trait ChannelManager {
    fn join(&self, ws: WS, channel: &str, message: PusherMessage) -> JoinResponse;
    fn leave(&self, channel: &str) -> LeaveResponse;
}
