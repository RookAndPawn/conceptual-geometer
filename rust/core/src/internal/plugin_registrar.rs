use crate::ConceptualGeometerPlugin;


pub trait PluginRegistrar {
    fn register_plugin(
        &mut self,
        name: &str,
        plugin: Box<dyn ConceptualGeometerPlugin>);
}
