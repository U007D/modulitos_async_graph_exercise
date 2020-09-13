use super::State;
use crate::node_traverser::NodeId;
use async_std::sync::{Arc, RwLock};
use bool_ext::BoolExt;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct Ids {
    // `HashMap` alternatives: (concurrent Hashmaps):
    // `CHashMap` seems to provide no way to iterate over the entire map without cloning it.
    // `evmap` requires its own `ShallowCopy` trait to implemented on Values, thus leaking the map
    //         abstraction.  `ShallowCopy` also defines one `unsafe` fn which must be implemented.
    // 'Dashmap' may deadlock on read if a mutable reference is held (which is odd for a concurrent
    //           HashMap, since all references should be read refs) and may deadlock on write if any
    //           other reference exists.  I may not be understanding the documentation correctly but
    //           those constraints seem to make it identical to HashMap.  There are also >100 unsafe
    //           functions in its implementation (according to `cargo geiger`).
    kvps: Arc<RwLock<HashMap<NodeId, State>>>,
}

impl Ids {
    pub fn new() -> Self {
        Self {
            kvps: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    #[inline]
    // Idiomatically, key lookup occurs by ref (in case key is not `Copy`).
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub async fn contains_key(&self, key: &NodeId) -> bool {
        self.kvps.read().await.contains_key(key)
    }

    #[inline]
    pub async fn is_empty(&self) -> bool { self.kvps.read().await.is_empty() }

    pub async fn iter_unvisited(&self) -> impl Iterator<Item = (NodeId, State)> {
        let iter = self
            .kvps
            .write()
            .await
            .iter_mut()
            .filter_map(|(&k, &mut mut v)| {
                (v == State::Unvisited).some_with(|| {
                    let res = (k, v);
                    v = State::Visited;
                    res
                })
            })
            .collect::<Vec<_>>()
            .into_iter();
        iter
    }

    #[inline]
    pub async fn len(&self) -> usize { self.kvps.read().await.len() }

    pub async fn push_unique(&self, key: NodeId) -> &Self {
        let mut w_guard = self.kvps.write().await;
        w_guard
            .contains_key(&key)
            .or_do(|| {
                w_guard.insert(key, State::Unvisited);
            });
        self
    }
}
