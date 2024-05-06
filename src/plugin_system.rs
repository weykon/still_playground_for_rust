struct PluginSystem {
    plugins: Vec<Box<dyn Plugin>>,
}
trait Plugin {
    fn func(&self);
}
struct PluginMeta {
    id: i32,
    name: String,
}
impl Plugin for PluginMeta {
    fn func(&self) {
        println!("PluginMeta: {} {}", self.id, self.name);
    }
}

impl PluginSystem {
    fn new() -> PluginSystem {
        PluginSystem {
            plugins: Vec::new(),
        }
    }
    fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }
    fn run_plugins(&self) {
        for plugin in &self.plugins {
            plugin.func();
        }
    }
}

trait Display {
    fn display(&self);
}

impl Display for PluginMeta {
    fn display(&self) {
        println!("Display: {} {}", self.id, self.name);
    }
}

pub(crate) fn run() {
    let mut plugin_system = PluginSystem::new();
    let rust_analyzer = Box::new(PluginMeta {
        id: 1,
        name: "rust-analyzer".to_string(),
    });
    let typescript_lsp = Box::new(PluginMeta {
        id: 2,
        name: "typescript-lsp".to_string(),
    });
    plugin_system.add_plugin(rust_analyzer);
    plugin_system.add_plugin(typescript_lsp);
    plugin_system.run_plugins();
}
