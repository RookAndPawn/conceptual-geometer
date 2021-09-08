use conceptual_geometer_core::{ConceptualGeometerPlugin, PluginRegistrar};

conceptual_geometer_core::export_plugin!(register);

#[allow(improper_ctypes_definitions)]
extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.register_function("Console", Box::new(ConsoleLogger));
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConsoleLogger;

impl ConceptualGeometerPlugin for ConsoleLogger {
    fn name(&self) -> &'static str {
        "Console Logger"
    }
}
