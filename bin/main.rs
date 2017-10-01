extern crate mackerel_plugin;
extern crate mackerel_plugin_loadavg;

use mackerel_plugin::Plugin;
use mackerel_plugin_loadavg::LoadavgPlugin;

fn main() {
    let plugin = LoadavgPlugin {};
    match plugin.run() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("mackerel-plugin-loadavg: {}", err);
            std::process::exit(1);
        }
    }
}
