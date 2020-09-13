use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Eq, Hash,PartialEq)]
pub struct Node {
    children: Vec<NodeId>,
    reward: i32,
}

impl Node {
    pub const fn children(&self) -> &Vec<NodeId> {
        &self.children
    }
    pub const fn reward(&self) -> i32 {
        self.reward
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NodeId(char);

impl From<char> for NodeId {
    fn from(c: char) -> Self {
        Self(c)
    }
}
