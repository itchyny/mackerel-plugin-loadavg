use mackerel_plugin::Plugin;
use mackerel_plugin_loadavg::LoadavgPlugin;

fn main() {
    if let Err(err) = (LoadavgPlugin {}).run() {
        eprintln!("mackerel-plugin-loadavg: {}", err);
        std::process::exit(1);
    }
}
