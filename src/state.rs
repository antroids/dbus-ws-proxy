use crate::message::OwnedSignalKey;
use std::hash::Hash;
use tokio::sync::Mutex;
use tokio_stream::{Stream, StreamExt, StreamMap};
use zbus::proxy::SignalStream;

#[derive(Default, Debug)]
pub struct WebSocketState {
    signals: StreamMapState<OwnedSignalKey, SignalStream<'static>>,
}

#[derive(Debug)]
pub struct StreamMapState<K, S>(Mutex<StreamMap<K, S>>);

impl<K, S> StreamMapState<K, S>
where
    K: Hash + Eq + Clone + Unpin,
    S: Stream + Unpin,
{
    pub async fn insert(&self, key: K, value: S) -> Option<S> {
        self.0.lock().await.insert(key, value)
    }

    pub async fn remove(&self, key: &K) -> Option<S> {
        self.0.lock().await.remove(key)
    }

    pub async fn next(&self) -> Option<(K, S::Item)> {
        self.0.lock().await.next().await
    }
}

impl<K, S> Default for StreamMapState<K, S> {
    fn default() -> Self {
        Self(Mutex::new(StreamMap::default()))
    }
}

impl WebSocketState {
    pub fn signals(&self) -> &StreamMapState<OwnedSignalKey, SignalStream<'static>> {
        &self.signals
    }
}
