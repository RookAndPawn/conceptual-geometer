use conceptual_geometer_core::{ConceptualGeometerPlugin};
use conceptual_geometer_core::model::CgNode;

conceptual_geometer_core::export_plugin!{
    "Console Logger" => {
        let node = CgNode {
            name: "Logger Root".to_owned()
        };

        ConsoleLogger {
            root_node: node
        }
    }
}



#[derive(Debug, Clone)]
pub struct ConsoleLogger {
    root_node: CgNode
}

impl ConceptualGeometerPlugin for ConsoleLogger {

    fn get_root_node(&self) -> &'_ CgNode {
        &self.root_node
    }

    fn get_nodes_by_id(&self) -> Vec<&'_ CgNode> {
        todo!()
    }
}
