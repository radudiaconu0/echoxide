use crate::adapters::adapter_trait::AdapterInterface;
use crate::adapters::local_adapter::LocalAdapter;
use crate::channels::channel::Channel;
use crate::channels::presence_channel_manager::PresenceMemberInfo;
use crate::namespace::Namespace;
use crate::ws_handler::WebSocket;
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

struct Adapter {
    pub driver: Arc<Mutex<dyn AdapterInterface>>,
    pub namespaces: Arc<Mutex<HashMap<String, Namespace>>>,
}

impl Adapter {
    pub fn new() -> Self {
        let driver = Arc::new(Mutex::new(LocalAdapter::new()));
        Adapter {
            driver,
            namespaces: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl AdapterInterface for Adapter {
    async fn get_namespace(&mut self, app_id: &str) -> Arc<&Namespace> {
        let mut driver = self.driver.lock().unwrap();
        driver.get_namespace(app_id).await
    }

    async fn get_namespaces(&self) -> HashMap<String, &Namespace> {
        let mut driver = self.driver.lock().unwrap();
        driver.get_namespaces().await
    }

    async fn add_socket(&mut self, app_id: &str, ws: WebSocket) -> bool {
        let mut driver = self.driver.lock().unwrap();
        driver.add_socket(app_id, ws).await
    }

    async fn remove_socket(&mut self, app_id: &str, ws_id: &str) -> bool {
        let mut driver = self.driver.lock().unwrap();
        driver.remove_socket(app_id, ws_id).await
    }

    async fn add_to_channel(&mut self, app_id: &str, channel: &str, ws: WebSocket) -> usize {
        let mut driver = self.driver.lock().unwrap();
        driver.add_to_channel(app_id, channel, ws).await
    }

    async fn remove_from_channel(
        &mut self,
        app_id: &str,
        channel: Channel,
        ws_id: &str,
    ) -> Option<usize> {
        let mut driver = self.driver.lock().unwrap();
        driver.remove_from_channel(app_id, channel, ws_id).await
    }

    async fn send(&mut self, app_id: &str, channel: &str, data: &str, excepting_id: Option<&str>) {
        let mut driver = self.driver.lock().unwrap();
        driver.send(app_id, channel, data, excepting_id).await
    }

    async fn terminate_user_connections(&mut self, app_id: &str, user_id: &str) {
        let mut driver = self.driver.lock().unwrap();
        driver.terminate_user_connections(app_id, user_id).await
    }

    async fn disconnect(&self) {
        let mut driver = self.driver.lock().unwrap();
        driver.disconnect().await
    }

    async fn clear_namespace(&mut self, namespace_id: &str) {
        let mut driver = self.driver.lock().unwrap();
        driver.clear_namespace(namespace_id).await
    }

    async fn clear_namespaces(&mut self) {
        let mut driver = self.driver.lock().unwrap();
        driver.clear_namespaces().await
    }

    async fn get_sockets(&mut self, app_id: &str, only_local: bool) -> HashMap<String, WebSocket> {
        let mut driver = self.driver.lock().unwrap();
        driver.get_sockets(app_id, only_local).await
    }

    async fn get_sockets_count(&mut self, app_id: &str, only_local: bool) -> usize {
        let mut driver = self.driver.lock().unwrap();
        driver.get_sockets_count(app_id, only_local).await
    }

    async fn get_channels(
        &mut self,
        app_id: &str,
        only_local: bool,
    ) -> HashMap<String, HashSet<String>> {
        let mut driver = self.driver.lock().unwrap();
        driver.get_channels(app_id, only_local).await
    }

    async fn get_channels_with_sockets_count(
        &mut self,
        app_id: &str,
        only_local: bool,
    ) -> HashMap<String, usize> {
        let mut driver = self.driver.lock().unwrap();
        driver
            .get_channels_with_sockets_count(app_id, only_local)
            .await
    }

    async fn get_channel_sockets(
        &mut self,
        app_id: &str,
        channel: &str,
        only_local: bool,
    ) -> HashMap<String, &WebSocket> {
        let mut driver = self.driver.lock().unwrap();
        driver
            .get_channel_sockets(app_id, channel, only_local)
            .await
    }

    async fn get_channel_sockets_count(
        &mut self,
        app_id: &str,
        channel: &str,
        only_local: bool,
    ) -> usize {
        let mut driver = self.driver.lock().unwrap();
        driver
            .get_channel_sockets_count(app_id, channel, only_local)
            .await
    }

    async fn get_channel_members(
        &mut self,
        app_id: &str,
        channel: &str,
        only_local: bool,
    ) -> HashMap<String, PresenceMemberInfo> {
        let mut driver = self.driver.lock().unwrap();
        driver
            .get_channel_members(app_id, channel, only_local)
            .await
    }

    async fn get_channel_members_count(
        &mut self,
        app_id: &str,
        channel: &str,
        only_local: bool,
    ) -> usize {
        let mut driver = self.driver.lock().unwrap();
        driver
            .get_channel_members_count(app_id, channel, only_local)
            .await
    }

    async fn is_in_channel(
        &mut self,
        app_id: &str,
        channel: &str,
        ws_id: &str,
        only_local: bool,
    ) -> bool {
        let mut driver = self.driver.lock().unwrap();
        driver
            .is_in_channel(app_id, channel, ws_id, only_local)
            .await
    }

    async fn add_user(&mut self, ws: WebSocket) {
        let mut driver = self.driver.lock().unwrap();
        driver.add_user(ws).await
    }

    async fn remove_user(&mut self, ws: WebSocket) {
        let mut driver = self.driver.lock().unwrap();
        driver.remove_user(ws).await
    }

    async fn get_user_sockets(&mut self, app_id: &str, user_id: &str) -> HashSet<&WebSocket> {
        let mut driver = self.driver.lock().unwrap();
        driver.get_user_sockets(app_id, user_id).await
    }
}
