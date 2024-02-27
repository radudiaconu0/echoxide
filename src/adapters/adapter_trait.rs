use crate::channels::channel::Channel;
use crate::channels::presence_channel_manager::PresenceMemberInfo;
use crate::namespace::Namespace;
use crate::ws_handler::WebSocket;
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

#[async_trait]
pub trait AdapterInterface: Send + Sync {
    // Assume Namespace, WebSocket, and PresenceMemberInfo are already defined
    // fn init(&self) -> Box<dyn AdapterInterface>;
    async fn get_namespace(&mut self, app_id: &str) -> Namespace;
    async fn get_namespaces(&self) -> HashMap<String, &Namespace>;
    async fn add_socket(&mut self, app_id: &str, ws: WebSocket) -> bool;
    async fn remove_socket(&mut self, app_id: &str, ws_id: &str) -> bool;
    async fn add_to_channel(&mut self, app_id: &str, channel: &str, ws: WebSocket) -> usize;
    async fn remove_from_channel(
        &mut self,
        app_id: &str,
        channel: Channel,
        ws_id: &str,
    ) -> Option<usize>;
    async fn send(&mut self, app_id: &str, channel: &str, data: &str, excepting_id: Option<&str>);
    async fn terminate_user_connections(&mut self, app_id: &str, user_id: &str);
    async fn disconnect(&self);
    async fn clear_namespace(&mut self, namespace_id: &str);
    async fn clear_namespaces(&mut self);
    async fn get_sockets(&mut self, app_id: &str, only_local: bool) -> HashMap<String, WebSocket>;
    async fn get_sockets_count(&mut self, app_id: &str, only_local: bool) -> usize;
    async fn get_channels(
        &mut self,
        app_id: &str,
        only_local: bool,
    ) -> HashMap<String, HashSet<String>>;
    async fn get_channels_with_sockets_count(
        &mut self,
        app_id: &str,
        only_local: bool,
    ) -> HashMap<String, usize>;
    async fn get_channel_sockets(
        &mut self,
        app_id: &str,
        channel: &str,
        only_local: bool,
    ) -> HashMap<String, &WebSocket>;
    async fn get_channel_sockets_count(
        &mut self,
        app_id: &str,
        channel: &str,
        only_local: bool,
    ) -> usize;
    async fn get_channel_members(
        &mut self,
        app_id: &str,
        channel: &str,
        only_local: bool,
    ) -> HashMap<String, PresenceMemberInfo>;
    async fn get_channel_members_count(
        &mut self,
        app_id: &str,
        channel: &str,
        only_local: bool,
    ) -> usize;
    async fn is_in_channel(
        &mut self,
        app_id: &str,
        channel: &str,
        ws_id: &str,
        only_local: bool,
    ) -> bool;
    async fn add_user(&mut self, ws: WebSocket);
    async fn remove_user(&mut self, ws: WebSocket);
    async fn get_user_sockets(&mut self, app_id: &str, user_id: &str) -> HashSet<&WebSocket>;
}
