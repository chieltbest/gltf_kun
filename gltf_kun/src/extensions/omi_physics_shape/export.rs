use crate::{
    extensions::ExtensionExport,
    graph::{gltf::document::GltfDocument, ByteNode, Extensions, Graph},
    io::format::gltf::GltfFormat,
};

use super::{json::RootExtension, OmiPhysicsShape, EXTENSION_NAME};

impl ExtensionExport<GltfDocument, GltfFormat> for OmiPhysicsShape {
    fn export(
        graph: &mut Graph,
        doc: &GltfDocument,
        format: &mut GltfFormat,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ext = match doc.get_extension::<Self>(graph) {
            Some(ext) => ext,
            None => return Ok(()),
        };

        let shapes = ext
            .shapes(graph)
            .map(|shape| shape.read(graph).into())
            .collect::<Vec<_>>();

        if shapes.is_empty() {
            return Ok(());
        }

        let root_extension = RootExtension { shapes };

        let extensions = format
            .json
            .extensions
            .get_or_insert(gltf::json::extensions::Root::default());

        extensions.others.insert(
            EXTENSION_NAME.to_string(),
            serde_json::to_value(root_extension)?,
        );

        format.json.extensions_used.push(EXTENSION_NAME.to_string());

        Ok(())
    }
}
