use std::io::Cursor;

use mackerel_plugin::Plugin;
use mackerel_plugin_loadavg::LoadavgPlugin;

#[test]
fn plugin_output_values() {
    let plugin = LoadavgPlugin {};
    let mut out = Cursor::new(Vec::new());
    assert!(plugin.output_values(&mut out).is_ok());
    let out_str = String::from_utf8(out.into_inner()).unwrap();
    assert!(out_str.contains("loadavg.loadavg1\t"));
    assert!(out_str.contains("loadavg.loadavg5\t"));
    assert!(out_str.contains("loadavg.loadavg15\t"));
}

#[test]
fn plugin_output_definitions() {
    let plugin = LoadavgPlugin {};
    let mut out = Cursor::new(Vec::new());
    assert!(plugin.output_definitions(&mut out).is_ok());
    let out_str = String::from_utf8(out.into_inner()).unwrap();
    assert!(out_str.starts_with("# mackerel-agent-plugin\n"));
}
