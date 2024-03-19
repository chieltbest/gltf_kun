use petgraph::graph::NodeIndex;

use crate::graph::{Extensions, GraphNodeWeight, Weight};

use super::GltfWeight;

#[derive(Clone, Debug, Default)]
pub struct BufferWeight {
    pub name: Option<String>,
    pub extras: gltf::json::Extras,

    pub uri: Option<String>,
}

impl From<BufferWeight> for Weight {
    fn from(weight: BufferWeight) -> Self {
        Self::Gltf(GltfWeight::Buffer(weight))
    }
}

impl<'a> TryFrom<&'a Weight> for &'a BufferWeight {
    type Error = ();
    fn try_from(value: &'a Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Buffer(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a mut Weight> for &'a mut BufferWeight {
    type Error = ();
    fn try_from(value: &'a mut Weight) -> Result<Self, Self::Error> {
        match value {
            Weight::Gltf(GltfWeight::Buffer(weight)) => Ok(weight),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Buffer(pub NodeIndex);

impl From<NodeIndex> for Buffer {
    fn from(index: NodeIndex) -> Self {
        Self(index)
    }
}

impl From<Buffer> for NodeIndex {
    fn from(buffer: Buffer) -> Self {
        buffer.0
    }
}

impl GraphNodeWeight<BufferWeight> for Buffer {}
impl Extensions for Buffer {}
