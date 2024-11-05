use axum::extract::FromRef;
use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
    sync::Arc,
};

use crate::storage::StoragePool;

#[derive(Clone, Debug)]
pub(crate) struct FeedControl {
    pub(crate) deny: String,
    pub(crate) allowed: HashSet<String>,
}

pub struct InnerWebContext {
    pub(crate) pool: StoragePool,
    pub(crate) external_base: String,
    pub(crate) feeds: HashMap<String, FeedControl>,
}

#[derive(Clone, FromRef)]
pub struct WebContext(pub(crate) Arc<InnerWebContext>);

impl Deref for WebContext {
    type Target = InnerWebContext;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl WebContext {
    pub fn new(
        pool: StoragePool,
        external_base: &str,
        feeds: HashMap<String, (String, HashSet<String>)>,
    ) -> Self {
        let feeds = feeds
            .into_iter()
            .map(|(uri, (deny, allowed))| (uri, FeedControl { deny, allowed }))
            .collect();
        Self(Arc::new(InnerWebContext {
            pool,
            external_base: external_base.to_string(),
            feeds,
        }))
    }
}