extern crate libc;
#[macro_use]
extern crate mackerel_plugin;

use std::collections::HashMap;
use mackerel_plugin::*;

pub struct LoadavgPlugin {}

#[inline]
fn get_loadavgs() -> Result<[f64; 3], String> {
    let mut loadavgs: [f64; 3] = [0.0, 0.0, 0.0];
    unsafe {
        let ret = libc::getloadavg(loadavgs.as_mut_ptr(), 3);
        if ret != 3 {
            Err("failed to get load averages".to_string())
        } else {
            Ok(loadavgs)
        }
    }
}

impl Plugin for LoadavgPlugin {
    fn fetch_metrics(&self) -> Result<HashMap<String, f64>, String> {
        let mut metrics = HashMap::new();
        let loadavgs = get_loadavgs()?;
        metrics.insert("loadavg.loadavg1".to_string(), loadavgs[0]);
        metrics.insert("loadavg.loadavg5".to_string(), loadavgs[1]);
        metrics.insert("loadavg.loadavg15".to_string(), loadavgs[2]);
        Ok(metrics)
    }

    fn graph_definition(&self) -> Vec<Graph> {
        vec![
            graph! {
                name: "loadavg",
                label: "Load averages",
                unit: "float",
                metrics: [
                    { name: "loadavg15", label: "loadavg15" },
                    { name: "loadavg5", label: "loadavg5" },
                    { name: "loadavg1", label: "loadavg1" },
                ]
            },
        ]
    }
}
