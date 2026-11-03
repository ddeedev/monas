use crate::TabData;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub String);

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl NodeId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeData {
    Folder { children: Vec<NodeId>, expand: bool },
    Tab { data: TabData, open: bool },
}

impl NodeData {
    pub fn new_tab_node(data: TabData, open: bool) -> Self {
        Self::Tab { data, open }
    }

    pub fn new_folder_node(children: Vec<NodeId>, expand: bool) -> Self {
        Self::Folder { children, expand }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub name: String,
    pub data: NodeData,
    // arrange location;
    pub position: u8,
}

impl Node {
    pub fn new(name: String, data: NodeData, position: u8) -> Self {
        Self {
            id: NodeId::default(),
            name,
            data,
            position,
        }
    }

    pub fn children(&self) -> &[NodeId] {
        match &self.data {
            NodeData::Folder { children, .. } => children,
            NodeData::Tab { .. } => &[],
        }
    }

    pub fn next_position(&self) -> u8 {
        u8::try_from(self.children().len() + 1).unwrap_or(u8::MAX)
    }
}

impl NodeId {
    pub fn new() -> Self {
        NodeId(Uuid::new_v4().to_string())
    }
}

pub trait Folder {
    fn has_child_folder(&self) -> bool;
    fn list_child_folder(&self) -> Vec<NodeId>;
}

impl Folder for Node {
    fn has_child_folder(&self) -> bool {
        match &self.data {
            NodeData::Tab { data: _, open: _ } => false,
            NodeData::Folder {
                children: _,
                expand: _,
            } => true,
        }
    }

    fn list_child_folder(&self) -> Vec<NodeId> {
        match &self.data {
            NodeData::Folder { children, .. } => children.clone(),
            NodeData::Tab { .. } => Vec::new(),
        }
    }
}
