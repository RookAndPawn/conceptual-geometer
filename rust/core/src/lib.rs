pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

pub use conceptual_geometer_plugin::ConceptualGeometerPlugin;


mod model;
mod conceptual_geometer_plugin;
pub mod internal;

#[macro_export]
macro_rules! export_plugin {
    ($plugin_name:expr => $plugin_constructor:block) => {

        mod private {
            use super::*;
            use ::conceptual_geometer_core::internal::{
                PluginDeclaration,
                PluginRegistrar
            };

            #[allow(improper_ctypes_definitions)]
            extern "C" fn register(
                registrar: &mut dyn PluginRegistrar)
            {
                let plugin_name : String = $plugin_name.into();
                let plugin = $plugin_constructor;
                registrar.register_plugin(&*plugin_name, Box::new(plugin));
            }

            #[doc(hidden)]
            #[no_mangle]
            pub static plugin_declaration: PluginDeclaration =
                PluginDeclaration {
                    rustc_version: $crate::RUSTC_VERSION,
                    core_version: $crate::CORE_VERSION,
                    register: private::register,
                };
        }
    };
}