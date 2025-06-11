use std::collections::HashMap;

use inkwell::values::FunctionValue;
use pyo3::{pyclass, pymethods};

use crate::bb_stats::BBStats;

#[pyclass]
#[derive(Clone)]
pub struct FunctionStats {
    pub name: String,
    pub basic_block_stats: HashMap<String, BBStats>,
}

impl FunctionStats {
    pub fn new(function: &FunctionValue) -> Self {
        let basic_block_stats = function
            .get_basic_blocks()
            .into_iter()
            .map(|bb| {
                let stats = BBStats::new(&bb);
                (stats.name.clone(), stats)
            })
            .collect();
        Self {
            name: function.get_name().to_string_lossy().to_string(),
            basic_block_stats,
        }
    }
}

#[pymethods]
impl FunctionStats {
    #[getter]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[getter]
    pub fn basic_block_stats(&self) -> HashMap<String, BBStats> {
        self.basic_block_stats.clone()
    }
}