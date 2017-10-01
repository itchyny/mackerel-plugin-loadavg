extern crate mackerel_plugin;
extern crate mackerel_plugin_loadavg;

use mackerel_plugin::Plugin;
use mackerel_plugin_loadavg::LoadavgPlugin;

use std::io::Cursor;

#[test]
fn plugin_output_values() {
    let plugin = LoadavgPlugin {};
    let mut out = Cursor::new(Vec::new());
    assert_eq!(plugin.output_values(&mut out).is_ok(), true);
    let out_str = String::from_utf8(out.into_inner()).unwrap();
    assert_eq!(out_str.contains("loadavg.loadavg1\t"), true);
    assert_eq!(out_str.contains("loadavg.loadavg5\t"), true);
    assert_eq!(out_str.contains("loadavg.loadavg15\t"), true);
}

#[test]
fn plugin_output_definitions() {
    let plugin = LoadavgPlugin {};
    let mut out = Cursor::new(Vec::new());
    assert_eq!(plugin.output_definitions(&mut out).is_ok(), true);
    let out_str = String::from_utf8(out.into_inner()).unwrap();
    assert_eq!(out_str.starts_with("# mackerel-agent-plugin\n"), true);
}
