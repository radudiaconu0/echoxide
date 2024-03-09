use crate::adapters::adapter::Adapter;
use crate::channels::channel::Channel;
use crate::channels::presence_channel_manager::PresenceMemberInfo;
use crate::namespace::Namespace;
use crate::ws_handler::WebSocket;
use async_trait::async_trait;
use aws_sdk_lambda::config::IntoShared;
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

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

impl Adapter for LocalAdapter {
    async fn get_namespace(&mut self, app_id: &str) -> Result<&mut Namespace, ()> {
        if self.namespaces.get_mut(app_id).is_none() {
            self.namespaces
                .insert(app_id.to_string(), Namespace::new(app_id.to_string()));
        }
        Ok(self.namespaces.get_mut(app_id).unwrap())
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
        namespace.unwrap().add_socket(ws).unwrap()
    }

    async fn remove_socket(&mut self, app_id: &str, ws_id: &str) -> bool {
        let mut namespace = self.get_namespace(app_id).await.unwrap();
        namespace.remove_socket(ws_id.to_string()).is_ok()
    }

    async fn add_to_channel(&mut self, app_id: &str, channel: &str, ws: WebSocket) -> usize {
        let mut namespace = self.get_namespace(app_id).await.unwrap();
        namespace
            .add_to_channel(channel.to_string(), ws.id.unwrap().clone())
            .await
            .expect("TODO: panic message");
        self.get_channel_sockets_count(app_id, channel, false).await
    }

    async fn remove_from_channel(
        &mut self,
        app_id: &str,
        channel: Channel,
        ws_id: &str,
    ) -> Result<usize, ()> {
        let namespace = self.get_namespace(app_id).await.unwrap();
        namespace.remove_from_channel(ws_id, channel).await
    }

    async fn send(mut self, app_id: &str, channel: &str, data: &str, excepting_id: Option<&str>) {
        // if channel.contains("#server-to-") {
        //     let user_id = channel.replace("#server-to-", "");
        //     let user_sockets = self.get_user_sockets(app_id, &user_id).await;
        //     for ws in user_sockets.lock().unwrap().deref_mut().into_iter() {
        //         ws.send_json(serde_json::Value::String(data.to_string()))
        //             .await;
        //     }
        //     return;
        // }
        let namespace = self.get_namespace(app_id).await.unwrap();
        let channel_sockets = namespace.get_channel_sockets(channel).unwrap();
        for (ws_id, mut ws) in channel_sockets {
            if excepting_id.is_some() && excepting_id.unwrap() == ws_id {
                continue;
            }
            ws.send_json(serde_json::Value::String(data.to_string()))
                .await;
        }
    }

    async fn terminate_user_connections(&mut self, app_id: &str, user_id: &str) {
        todo!("Implement terminate_user_connections")
    }

    async fn disconnect(&self) {
        return;
    }

    async fn clear_namespace(&mut self, namespace_id: &str) {
        self.namespaces.insert(
            namespace_id.to_string(),
            Namespace::new(namespace_id.to_string()),
        );
        return;
    }

    async fn clear_namespaces(&mut self) {
        self.namespaces.clear();
        return;
    }

    async fn get_sockets(
        &mut self,
        app_id: &str,
        only_local: bool,
    ) -> Arc<Mutex<HashMap<String, WebSocket>>> {
        let namespace = self.get_namespace(app_id).await.unwrap();
        Arc::clone(&namespace.get_sockets().unwrap())
    }

    async fn get_sockets_count(&mut self, app_id: &str, only_local: bool) -> usize {
        let namespace = self.get_namespace(app_id).await.unwrap();
        namespace.get_sockets().unwrap().lock().unwrap().len()
    }

    async fn get_channels(
        &mut self,
        app_id: &str,
        only_local: bool,
    ) -> HashMap<String, HashSet<String>> {
        let mut namespace = self.get_namespace(app_id).await.unwrap();
        namespace.get_channels().unwrap()
    }

    async fn get_channels_with_sockets_count(
        &mut self,
        app_id: &str,
        only_local: bool,
    ) -> HashMap<String, usize> {
        let mut namespace = self.get_namespace(app_id).await.unwrap();
        namespace.get_channels_with_sockets_count().unwrap()
    }

    async fn get_channel_sockets(
        &mut self,
        app_id: &str,
        channel: &str,
        only_local: bool,
    ) -> HashMap<String, &WebSocket> {
        let namespace = self.get_namespace(app_id).await.unwrap();
        namespace.get_channel_sockets(channel).unwrap()
    }

    async fn get_channel_sockets_count(
        &mut self,
        app_id: &str,
        channel: &str,
        only_local: bool,
    ) -> usize {
        let namespace = self.get_namespace(app_id).await.unwrap();
        namespace.get_channel_sockets(channel).unwrap().len()
    }

    async fn get_channel_members(
        &mut self,
        app_id: &str,
        channel: &str,
        only_local: bool,
    ) -> HashMap<String, PresenceMemberInfo> {
        let namespace = self.get_namespace(app_id).await.unwrap();
        namespace.get_channel_members(channel).unwrap()
    }

    async fn get_channel_members_count(
        &mut self,
        app_id: &str,
        channel: &str,
        only_local: bool,
    ) -> usize {
        let namespace = self.get_namespace(app_id).await.unwrap();
        namespace.get_channel_members(channel).unwrap().len()
    }

    async fn is_in_channel(
        &mut self,
        app_id: &str,
        channel: &str,
        ws_id: &str,
        only_local: bool,
    ) -> bool {
        let namespace = self.get_namespace(app_id).await.unwrap();
        namespace.is_in_channel(channel, ws_id).unwrap()
    }

    async fn add_user(&mut self, ws: WebSocket) {
        self.add_socket(ws.app_key.as_ref().unwrap(), ws).await;
    }

    async fn remove_user(&mut self, ws: WebSocket) {
        self.remove_socket(ws.app_key.as_ref().unwrap(), ws.id.as_ref().unwrap())
            .await;
    }

    async fn get_user_sockets(
        &mut self,
        app_id: &str,
        user_id: &str,
    ) -> Arc<Mutex<HashSet<&WebSocket>>> {
        return self
            .get_namespace(app_id)
            .await
            .unwrap()
            .get_user_sockets(user_id)
            .unwrap();
    }
    // fn init(&self) -> Box<dyn AdapterInterface> {
    //     Box::new(self)
    // }
}
