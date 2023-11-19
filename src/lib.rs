use mackerel_plugin::{graph, Graph, Plugin};
use std::collections::HashMap;

pub struct LoadavgPlugin {}

impl Plugin for LoadavgPlugin {
    fn fetch_metrics(&self) -> Result<HashMap<String, f64>, String> {
        let mut loadavgs: [f64; 3] = [0.0, 0.0, 0.0];
        if unsafe { libc::getloadavg(loadavgs.as_mut_ptr(), 3) } != 3 {
            return Err("failed to get load averages".to_owned());
        }
        Ok(HashMap::from([
            ("loadavg.loadavg1".to_owned(), loadavgs[0]),
            ("loadavg.loadavg5".to_owned(), loadavgs[1]),
            ("loadavg.loadavg15".to_owned(), loadavgs[2]),
        ]))
    }

    fn graph_definition(&self) -> Vec<Graph> {
        vec![graph! {
            name: "loadavg",
            label: "Load averages",
            unit: "float",
            metrics: [
                { name: "loadavg15", label: "loadavg15" },
                { name: "loadavg5", label: "loadavg5" },
                { name: "loadavg1", label: "loadavg1" },
            ],
        }]
    }
}
