use crate::channels::public_channel_manager::{JoinResponse, LeaveResponse};
use crate::message::PusherMessage;
use fastwebsockets::FragmentCollector;

pub trait ChannelManager<S> {
    fn join(
        &self,
        ws: FragmentCollector<S>,
        channel: &str,
        message: PusherMessage,
    ) -> JoinResponse<S>;
    fn leave(&self, channel: &str) -> LeaveResponse;
}
