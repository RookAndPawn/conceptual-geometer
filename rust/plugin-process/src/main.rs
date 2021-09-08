use libloading::Library;
use conceptual_geometer_core::{
    ConceptualGeometerPlugin,
    PluginDeclaration,
    PluginRegistrar as Registrar,
    RUSTC_VERSION,
    CORE_VERSION
};
use std::{alloc::System, env, ffi::OsStr, path::PathBuf};
use eyre::{Result, eyre};

#[global_allocator]
static ALLOCATOR: System = System;

fn main() {
    // parse arguments
    let args = env::args().skip(1);
    let args = Args::parse(args)
        .expect("Usage: plugin-process plugin-library");

    let process = unsafe {
        PluginProcess
            ::load(&args.plugin_library)
            .expect("Function loading failed")
    };

    println!(
        "name - {}",
        process.plugin.name(),
    );
}

struct Args {
    plugin_library: PathBuf,
    function: String,
    arguments: Vec<f64>,
}

impl Args {
    fn parse(mut args: impl Iterator<Item = String>) -> Option<Args> {
        let plugin_library = PathBuf::from(args.next()?);
        let function = args.next()?;
        let mut arguments = Vec::new();

        for arg in args {
            arguments.push(arg.parse().ok()?);
        }

        Some(Args {
            plugin_library,
            function,
            arguments,
        })
    }
}

/// Storage for the plugin associated with this process
pub struct PluginProcess {
    name: String,
    plugin: Box<dyn ConceptualGeometerPlugin>,
    library: Library
}

impl PluginProcess {
    pub fn new(name: String, plugin: Box<dyn ConceptualGeometerPlugin>, library: Library) -> PluginProcess {
        PluginProcess { name, plugin, library }
     }

    pub fn name(
        &self,
    ) -> &'static str {
        self.plugin.name()
    }

    /// Load a plugin library and add all contained functions to the internal
    /// function table.
    ///
    /// # Safety
    ///
    /// A plugin library **must** be implemented using the
    /// [`plugins_core::plugin_declaration!()`] macro. Trying manually implement
    /// a plugin without going through that macro will result in undefined
    /// behavior.
    pub unsafe fn load<P: AsRef<OsStr>>(
        library_path: P,
    ) -> Result<PluginProcess> {
        // load the library into memory
        let library = Library::new(library_path)?;

        // get a pointer to the plugin_declaration symbol.
        let decl = library
            .get::<*mut PluginDeclaration>(b"plugin_declaration\0")?
            .read();

        // version checks to prevent accidental ABI incompatibilities
        if decl.rustc_version != RUSTC_VERSION
            || decl.core_version != CORE_VERSION
        {
            return Err(eyre!("Version Mismatch"));
        }

        let mut registrar = PluginRegistrar::new(library);

        (decl.register)(&mut registrar);

        let PluginRegistrar { lib, mut plugin } = registrar;

        let RegisteredPlugin {
            plugin,
            name
        } = plugin
            .take()
            .expect("No plugin was registered");

        Ok(PluginProcess::new(name, plugin, lib))
    }
}

struct PluginRegistrar {
    lib: Library,
    plugin: Option<RegisteredPlugin>
}

impl PluginRegistrar {
    fn new(lib: Library) -> PluginRegistrar {
        PluginRegistrar {
            lib,
            plugin: None,
        }
    }
}

impl Registrar for PluginRegistrar {
    fn register_function(
        &mut self,
        name: &str,
        plugin: Box<dyn ConceptualGeometerPlugin>
    ) {

        let proxy = RegisteredPlugin {
            name: name.to_owned(),
            plugin,
        };

        self.plugin = Some(proxy);
    }
}

/// A proxy object which wraps a [`Function`] and makes sure it can't outlive
/// the library it came from.
pub struct RegisteredPlugin {
    name: String,
    plugin: Box<dyn ConceptualGeometerPlugin>,
}