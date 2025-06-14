use std::collections::HashMap;

use llvm_ir::Module;
use pyo3::{prelude::*, pyclass, pymethods, types::PyBytes, Bound, PyResult};
use rayon::prelude::*;

use crate::fn_features::FnFeatures;

#[pyclass]
pub struct ModFeatures {
    #[pyo3(get)]
    pub fn_feats: HashMap<String, FnFeatures>,
    #[pyo3(get)]
    pub call_sites: HashMap<String, Vec<String>>,
}

#[pymethods]
impl ModFeatures {
    #[new]
    pub fn new(bc: Bound<PyBytes>) -> PyResult<Self> {
        let module = Module::from_bc_bytes(bc.as_bytes())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        let fn_feats = module
            .functions
            .par_iter()
            .map(|func| {
                let stats = FnFeatures::new(&func);
                (stats.name.clone(), stats)
            })
            .collect();
        let call_sites = fn_feats.values().map(|func| {
            (func.name.clone(), func.calls.clone())
        }).collect();
        Ok(Self { fn_feats, call_sites })
    }
}
