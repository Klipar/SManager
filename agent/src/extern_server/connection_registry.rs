use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use futures::future::join_all;
use serde_json::Value;
use tokio::sync::{oneshot, Mutex};

use shared::server::message::Message;

type MessageTx = tokio::sync::mpsc::Sender<OutboundRequest>;

pub struct OutboundRequest {
    pub message: Message,
    pub response_tx: oneshot::Sender<Message>,
}

#[derive(Default)]
struct RegistryInner {
    connections: HashMap<i32, MessageTx>, // key - core_id
    groups: HashMap<String, HashSet<i32>>, // key - group_name, value - core_ids
    next_id: u64,
}

impl RegistryInner {
    fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);

        if self.next_id == 0{ // 0 is preserved for special undefined messages
            self.next_id += 1;
        }

        id
    }
}

#[derive(Clone, Default)]
pub struct ConnectionRegistry {
    inner: Arc<Mutex<RegistryInner>>,
}

impl ConnectionRegistry {
    pub async fn register(&self, core_id: i32, tx: MessageTx) -> bool {
        let mut inner = self.inner.lock().await;

        match inner.connections.entry(core_id) {
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(tx);
                true
            }
            std::collections::hash_map::Entry::Occupied(_) => false,
        }
    }

    pub async fn unregister(&self, core_id: i32) {
        let mut inner = self.inner.lock().await;
        inner.connections.remove(&core_id);
        for members in inner.groups.values_mut() {
            members.remove(&core_id);
        }
    }

    pub async fn join_group(&self, core_id: i32, group: &str) {
        self.inner.lock().await
            .groups
            .entry(group.to_string())
            .or_default()
            .insert(core_id);
    }

    pub async fn leave_group(&self, core_id: i32, group: &str) {
        if let Some(members) = self.inner.lock().await.groups.get_mut(group) {
            members.remove(&core_id);
        }
    }

    async fn send_to(
        &self,
        core_id: i32,
        action: &str,
        data: Option<Value>,
    ) -> Option<Message> {
        let (tx, id) = {
            let mut inner = self.inner.lock().await;
            let tx = inner.connections.get(&core_id).cloned()?;
            let id = inner.next_id();
            (tx, id)
        };

        let message = Message::Request {
            id,
            action: action.to_string(),
            data,
        };

        let (resp_tx, resp_rx) = oneshot::channel();
        tx.send(OutboundRequest { message, response_tx: resp_tx }).await.ok()?;

        tokio::time::timeout(Duration::from_secs(10), resp_rx)
            .await.ok()?.ok()
    }

    pub async fn broadcast_to_group(
        &self,
        group: &str,
        data: Option<Value>,
    ) -> HashMap<i32, Option<Message>> {
        let members: Vec<i32> = {
            let inner = self.inner.lock().await;
            inner.groups.get(group)
                .map(|s| s.iter().copied().collect())
                .unwrap_or_default()
        };

        let futures: Vec<_> = members.into_iter().map(|id| {
            let registry = self.clone();
            let group = group.to_string();
            let data = data.clone();
            async move {
                let result = registry.send_to(id, &group, data).await;
                (id, result)
            }
        }).collect();

        join_all(futures).await.into_iter().collect()
    }
}