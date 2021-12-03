use crate::model::v_0::CgNode;



pub trait ConceptualGeometerPlugin {
    fn get_root_node(&self) -> &'_ CgNode;

    fn get_nodes_by_id(&self) -> Vec<&'_ CgNode>;
}
