use std::collections::HashMap;

use pyo3::{prelude::*, pyclass, pymethods, types::PyBytes, Bound, PyResult};

use crate::{function_stats::FunctionStats, llvm_module::LLVMModule};


#[pyclass]
pub struct ModuleStats {
    pub function_stats: HashMap<String, FunctionStats>,
}

#[pymethods]
impl ModuleStats {
    #[new]
    pub fn new(bc: Bound<PyBytes>) -> PyResult<Self> {
        let module = LLVMModule::from_bitcode(bc.as_bytes()).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let function_stats = module
            .get_functions()
            .map(|func| {
                let stats = FunctionStats::new(&func);
                (stats.name.clone(), stats)
            })
            .collect();
        Ok(Self { function_stats })
    }

    #[getter]
    pub fn function_stats(&self) -> HashMap<String, FunctionStats> {
        self.function_stats.clone()
    }
}