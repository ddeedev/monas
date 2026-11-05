use crate::node::{Node, NodeId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpaceId(pub String);

impl SpaceId {
    pub fn new() -> Self {
        SpaceId(Uuid::new_v4().to_string())
    }
}

impl Default for SpaceId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceContext {
    pub id: SpaceId,
    // number use for cmd + number to switch space
    pub name: String,
    pub number: usize,
    pub profile: ProfileId,
    pub sidebar: SidebarContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileContext {
    pub id: ProfileId,
    // user_name// name tag
    pub pseudonym: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidebarContext {
    pub folder: Vec<NodeId>,
    pub favorites: Vec<NodeId>,
    pub pinned: Vec<NodeId>,
    pub nodes: HashMap<NodeId, Node>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProfileId(pub String);

impl ProfileId {
    pub fn new() -> Self {
        ProfileId(Uuid::new_v4().to_string())
    }
}

impl Default for ProfileId {
    fn default() -> Self {
        Self::new()
    }
}

impl ProfileContext {
    pub fn new(name: String, email: Option<String>) -> Self {
        Self {
            id: ProfileId::default(),
            pseudonym: name,
            email,
        }
    }
}

impl SidebarContext {
    pub fn new() -> Self {
        Self {
            folder: Vec::new(),
            favorites: Vec::new(),
            pinned: Vec::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn with_folder(mut self, folder: Vec<NodeId>) -> Self {
        self.folder = folder;
        self
    }

    pub fn with_pinned_tab(mut self, pinned: Vec<NodeId>) -> Self {
        self.pinned = pinned;
        self
    }

    pub fn with_favorite(mut self, favorites: Vec<NodeId>) -> Self {
        self.favorites = favorites;
        self
    }

    pub fn with_nodes(mut self, nodes: HashMap<NodeId, Node>) -> Self {
        self.nodes = nodes;
        self
    }

    pub fn new_sidebar(favorites: Vec<NodeId>, folders: Vec<NodeId>, all: Vec<Node>) -> Self {
        let nodes = all.into_iter().map(|n| (n.id.clone(), n)).collect();

        Self::new()
            .with_nodes(nodes)
            .with_favorite(favorites)
            .with_folder(folders)
    }
}

impl Default for SidebarContext {
    fn default() -> Self {
        Self::new()
    }
}

impl SpaceContext {
    pub fn new(name: String, number: usize, profile: String) -> Self {
        Self {
            id: SpaceId::default(),
            profile: ProfileId(profile),
            name,
            number,
            sidebar: SidebarContext::default(),
        }
    }
}
