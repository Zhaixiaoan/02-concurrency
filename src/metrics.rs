use std::{fmt, sync::Arc};

use anyhow::Result;
use dashmap::DashMap;

#[derive(Debug, Clone)]
pub struct Metrics {
    pub data: Arc<DashMap<String, f64>>,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(DashMap::new()),
        }
    }
    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        // let mut data = self.data..map_err(|e| anyhow!(e.to_string()))?;
        let mut counter = self.data.entry(key.into()).or_insert(0.0);
        *counter += 1.0;
        Ok(())
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}
impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
