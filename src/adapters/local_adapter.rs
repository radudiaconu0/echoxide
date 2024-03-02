use crate::adapters::adapter_trait::AdapterInterface;
use crate::channels::channel::Channel;
use crate::channels::presence_channel_manager::PresenceMemberInfo;
use crate::namespace::Namespace;
use crate::ws_handler::WebSocket;
use async_trait::async_trait;
use aws_sdk_lambda::config::IntoShared;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use tokio::sync::Mutex;

pub struct LocalAdapter {
    pub namespaces: HashMap<String, Namespace>,
}

impl LocalAdapter {
    pub fn new() -> Self {
        LocalAdapter {
            namespaces: HashMap::new(),
        }
    }
}

impl AdapterInterface for LocalAdapter {
    async fn get_namespace(&mut self, app_id: &str) -> Result<&Namespace, ()> {
        if self.namespaces.get(app_id).is_none() {
            self.namespaces
                .insert(app_id.to_string(), Namespace::new(app_id.to_string()));
        }
        Ok(self.namespaces.get(app_id).unwrap())
    }

    async fn get_namespaces(&self) -> HashMap<String, &Namespace> {
        // return namespaces without clone
        let namespaces = &self.namespaces;
        let mut result = HashMap::new();
        for (k, v) in namespaces.iter() {
            result.insert(k.clone(), v.clone());
        }
        result
    }

    async fn add_socket(&mut self, app_id: &str, ws: WebSocket) -> bool {
        let mut namespace = self.get_namespace(app_id).await;
        namespace.add_socket(ws)
    }

    async fn remove_socket(&mut self, app_id: &str, ws_id: &str) -> bool {
        let mut namespace = self.get_namespace(app_id).await;
        namespace.remove_socket(ws_id.to_string())
    }

    async fn add_to_channel(&mut self, app_id: &str, channel: &str, ws: WebSocket) -> usize {
        let mut namespace = self.get_namespace(app_id).await;
        namespace
            .add_to_channel(channel.to_string(), ws.id.unwrap().clone())
            .await;
        self.get_channel_sockets_count(app_id, channel, false).await
    }

    async fn remove_from_channel(
        &mut self,
        app_id: &str,
        channel: Channel,
        ws_id: &str,
    ) -> Option<usize> {
        let mut namespace = self.get_namespace(app_id).await;
        namespace.remove_from_channel(ws_id, channel).await
    }

    async fn send(&mut self, app_id: &str, channel: &str, data: &str, excepting_id: Option<&str>) {
        if channel.contains("#server-to-") {
            let user_id = channel.replace("#server-to-", "");
            let user_sockets = self.get_user_sockets(app_id, &user_id).await;
            for mut ws in user_sockets {
                ws.send_json(serde_json::Value::String(data.to_string()))
                    .await;
            }
            return;
        }
        let mut namespace = self.get_namespace(app_id).await;
        let channel_sockets = namespace.get_channel_sockets(channel);
        for mut ws in channel_sockets {
            if excepting_id.is_some() && ws.1.id.as_ref().unwrap() == excepting_id.unwrap() {
                return;
            }
            ws.1.send_json(serde_json::Value::String(data.to_string()))
                .await;
        }
    }

    async fn terminate_user_connections(&mut self, app_id: &str, user_id: &str) {
        self.get_namespace(app_id)
            .await
            .terminate_user_connections(user_id)
            .await;
    }

    async fn disconnect(&self) {
        return;
    }

    async fn clear_namespace(&mut self, namespace_id: &str) {
        self.namespaces.lock().await.insert(
            namespace_id.to_string(),
            Namespace::new(namespace_id.to_string()),
        );
        return;
    }

    async fn clear_namespaces(&mut self) {
        self.namespaces.lock().await.clear();
        return;
    }

    async fn get_sockets(&mut self, app_id: &str, only_local: bool) -> HashMap<String, WebSocket> {
        let mut namespace = self.get_namespace(app_id).await;
        namespace.get_sockets()
    }

    async fn get_sockets_count(&mut self, app_id: &str, only_local: bool) -> usize {
        let mut namespace = self.get_namespace(app_id).await;
        namespace.get_sockets().len()
    }

    async fn get_channels(
        &mut self,
        app_id: &str,
        only_local: bool,
    ) -> HashMap<String, HashSet<String>> {
        let mut namespace = self.get_namespace(app_id).await;
        namespace.get_channels()
    }

    async fn get_channels_with_sockets_count(
        &mut self,
        app_id: &str,
        only_local: bool,
    ) -> HashMap<String, usize> {
        let mut namespace = self.get_namespace(app_id).await;
        namespace.get_channels_with_sockets_count()
    }

    async fn get_channel_sockets(
        &mut self,
        app_id: &str,
        channel: &str,
        only_local: bool,
    ) -> HashMap<String, &WebSocket> {
        let namespace = self.get_namespace(app_id).await;
        namespace.get_channel_sockets(channel)
    }

    async fn get_channel_sockets_count(
        &mut self,
        app_id: &str,
        channel: &str,
        only_local: bool,
    ) -> usize {
        let namespace = self.get_namespace(app_id).await;
        namespace.get_channel_sockets(channel).len()
    }

    async fn get_channel_members(
        &mut self,
        app_id: &str,
        channel: &str,
        only_local: bool,
    ) -> HashMap<String, PresenceMemberInfo> {
        let namespace = self.get_namespace(app_id).await;
        namespace.get_channel_members(channel)
    }

    async fn get_channel_members_count(
        &mut self,
        app_id: &str,
        channel: &str,
        only_local: bool,
    ) -> usize {
        let namespace = self.get_namespace(app_id).await;
        namespace.get_channel_members(channel).len()
    }

    async fn is_in_channel(
        &mut self,
        app_id: &str,
        channel: &str,
        ws_id: &str,
        only_local: bool,
    ) -> bool {
        let namespace = self.get_namespace(app_id).await;
        namespace.is_in_channel(channel, ws_id)
    }

    async fn add_user(&mut self, ws: WebSocket) {
        self.add_socket(ws.app_key.as_ref().unwrap(), ws).await;
    }

    async fn remove_user(&mut self, ws: WebSocket) {
        self.remove_socket(ws.app_key.as_ref().unwrap(), ws.id.as_ref().unwrap())
            .await;
    }

    async fn get_user_sockets(&mut self, app_id: &str, user_id: &str) -> HashSet<&WebSocket> {
        return self.get_namespace(app_id).await.get_user_sockets(user_id);
    }
    // fn init(&self) -> Box<dyn AdapterInterface> {
    //     Box::new(self)
    // }
}
