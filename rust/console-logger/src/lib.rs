use conceptual_geometer_core::{ConceptualGeometerPlugin};

conceptual_geometer_core::export_plugin!("Console Logger" => {
    ConsoleLogger::default()
});

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ConsoleLogger;

impl ConceptualGeometerPlugin for ConsoleLogger {

    fn name(&self) -> &'static str {
        "Console Logger"
    }
}
