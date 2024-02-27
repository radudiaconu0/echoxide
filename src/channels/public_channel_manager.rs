use crate::message::PusherMessage;
use crate::utils;
use fastwebsockets::FragmentCollector;

pub struct JoinResponse<S> {
    pub(crate) server: FragmentCollector<S>,
    pub(crate) success: bool,
    pub(crate) channel_connections: Option<i64>,
    pub(crate) auth_error: Option<bool>,
    pub(crate) member: Option<bool>,
    pub(crate) error_message: Option<String>,
    pub(crate) error_code: Option<i64>,
    pub(crate) type_: Option<String>,
}

pub struct LeaveResponse {
    pub(crate) left: bool,
    pub(crate) remaining_connections: Option<i64>,
    pub(crate) member: Option<bool>,
}

struct PublicChannelManager<S> {
    server: FragmentCollector<S>,
}

impl<S> PublicChannelManager<S> {
    pub fn join(
        &self,
        server: FragmentCollector<S>,
        channel: &str,
        message: PusherMessage,
    ) -> JoinResponse<S> {
        if utils::Utils::restricted_channel_name(channel) {
            JoinResponse {
                server,
                success: false,
                channel_connections: Some(0),
                auth_error: Some(true),
                member: Some(false),
                error_message: Some("The channel name is not allowed. Read channel conventions: https://pusher.com/docs/channels/using_channels/channels/#channel-naming-conventions".parse().unwrap()),
                error_code: Some(4009),
                type_: Some("PusherError".to_string()),
            }
        } else {
            JoinResponse {
                server,
                success: true,
                channel_connections: Some(0),
                auth_error: Some(false),
                member: Some(false),
                error_message: None,
                error_code: None,
                type_: None,
            }
        }
    }

    pub fn leave(&self, channel: &str) -> LeaveResponse {
        LeaveResponse {
            left: true,
            remaining_connections: Some(0),
            member: None,
        }
    }
}
